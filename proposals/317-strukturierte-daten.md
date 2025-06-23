# GSC317: Kompatibler Austausch strukturierter Daten

Matrix unterstützt neben den eingebauten Events auch den Austausch proprietärer Events. Einzige
Bedingung ist, dass neue Event-Typen gemäß [Spezifikation] mit einem Vendor-Präfix beginnen müssen
um Kollisionen untereinandeer und mit den standardisierten `m.*` Events zu vermeiden. TI-M
Hersteller können daher schon heute eigene Event-Typen für den Austausch strukturierter Daten
verwenden und damit spezialisierte Anwendungsfälle bedienen. Ebenso ist es möglich strukturierte
Daten als Datei über den eingebauten `m.file` Message-Type mit entsprechendem MIME-Type zu
versenden.

Hierbei ergeben sich allerdings folgende Probleme:

1.  Kompatibilität – Während Clients desselben Herstellers in der Regel dieselben proprietären
    Inhalte unterstützen sollten, kann das für Clients verschiedener Hersteller nicht angenommen
    werden. Zusätzlich beinhaltet Matrix aktuell keine Mechanismen, mit denen sich feststellen ließe
    welche Event-Typen Clients unterstützen oder ob ein gesendetes Event vom Empfänger verstanden
    wurde. Im schlimmsten Fall wird ein unbekanntes Event beim Empfänger überhaupt nicht angezeigt
    oder verarbeitet, ohne dass der Absender Kenntnis davon erlangt. Das kann besonders dann fatal
    sein, wenn automatisierte Systeme als TI-M Clients auftreten.
2.  Discoverability – Um zu verhindern, dass verschiedene Hersteller vergleichbare Probleme mehrfach
    lösen wäre es wünschenswert eine zentrale Übersicht ähnlich zu den [Dienstkennung] von KIM zu
    haben.

Dieses Proposal befasst sich mit Punkt 1 und beschreibt ein Verfahren, mit dem TI-M Clients Events
mit proprietären Inhalten kompatibel austauschen können.

## Änderungsvorschlag

Zentrales Instrument dieses Proposals ist [MSC4300]. Dieses MSC definiert einen Mechanismus, mit dem
Absender den Verarbeitungsstatus eines Events von Empfängern anfordern können. Damit diese Funktion
sinnvoll nutzbar ist, werden alle TI-M Clients verpflichtet, solche Anfragen zu beantworten.

**A_1 - Antwort auf Anfragen zum Verarbeitungsstatus von Events**

TI-M Clients MÜSSEN Anfragen zum Verarbeitungsstatus von Events gemäß [MSC4300] beantworten. Enthält
ein Event einen `de.gematik.msc4300.request.status` Content-Block, so MUSS der empfangende Client
mit einem `de.gematik.msc4300.response.status` Event antworten sofern die `lifetime` der Anfrage
noch nicht verstrichen ist. **\[\<=\]**

Absendene Clients sollten Anfragen zum Verarbeitungsstatus nicht mit jedem beliebigen Event
verschicken sondern nur in Fällen, in denen die Kenntnis des Verarbeitungsstatus essentiell ist.
Dies wird üblicherweise nur bei proprietären Events oder Dateien mit speziellem Inhalt der Fall
sein. So wäre es z. B. unsinnig den Verarbeitungsstatus von reinen Text- oder Bild-Nachrichten zu
erfragen, da alle TI-M Clients diese Nachrichten ohnehin unterstützen sollten.

Im Falle, dass ein empfangender Client ein ihm unbekanntes proprietäres Event antrifft, ist es
essenziell, dass der Nutzer die Möglichkeit hat den Inhalt bei Bedarf manuell weiterzuverarbeiten.
Dazu ist es erforderlich, dass solche Events angezeigt und mit der Möglichkeit zur Einsicht der
Rohdaten versehen werden.

**A_2 - Anzeige und Rohdaten von unbekannten proprietären Events**

TI-M Clients MÜSSEN ihnen unbekannte proprietäre Events (also solche, deren Typ nicht mit `m.`
beginnt) darstellen und dem Nutzer die Möglichkeit bieten die Rohdaten des Events anzuzeigen.
**\[\<=\]**

*Offene Frage: Wäre es hilfreich die generelle Unterstützung eines Event-Typs schon vor dem Versand
mittels [MSC4301] anfragen zu können? Was wären konkrete Anwendungsfälle dafür?*

*Offene Frage: Viele Probleme des medizinischen Bereichs sind bereits mit FHIR modelliert worden. In
Matrix können FHIR-Dateien schon heute über den `m.file` Message-Type mit dem MIME-Type
`application/fhir+json` bzw. `application/fhir+xml` versendet werden. Wie schmerzhaft ist hierbei
der Umweg über eine Datei? Wäre es hilfreich, die Daten mittels [MSC4302] inline in Events versenden
zu können oder den Typ der enthaltenen Resourcen auch ohne Download erkennen zu können?*

### Beispiel

Dr. Alice und Patient Bob befinden sich in einem Matrix-Raum. Alice ist mit nur einem Client `A`
verbunden wohingegen Bob auf zwei Geräten `B1` und `B2` angemeldet ist. Alice schickt Bob einen
Anamnese-Fragebogen als FHIR [`Questionnaire`] in einer XML-Datei. Da Alice sicher gehen möchte,
dass Bob den Fragebogen auf seinem Client ausfüllen kann, fragt sie zusammen mit dem Event auch
dessen Verarbeitungsstatus an.

``` json5
{
  "event_id": "$1",
  "type": "m.room.message",
  "sender": "@dr.alice:alice.com",
  "content": {
    // Fragebogen
    "msgtype": "m.file",
    "body": "Bitten füllen sie diesen Fragebogen aus, danke.",
    "filename": "questionnaire.xml",
    "info": {
      "mimetype": "application/fhir+xml",
      ...
    },
    // Anfrage des Verarbeitungsstatus
    "de.gematik.msc4300.request.status": {
      "from_device": "A",
      "lifetime": 600_000, // 10min
    }
    ...
  },
  ...
}
```

Bobs Client `B1` ist eine ältere Version und unterstützt das Ausfüllen von FHIR [`Questionnaire`]s
nicht. Er unterstützt aber den allgemeinen Download von Dateien. Da dies bei FHIR-XML-Dokumenten
eine u. U. starke Einschränkung für den Nutzer bedeuten kann, antwortet er mit einer Warnung:

``` json5
{
  "type": "de.gematik.msc4300.response.status",
  "sender": "@bob:bob.com",
  "content": {
    "m.response.status": {
      "from_device": "B1",
      "status": "success",
      "messages": [{
        "type": "warning",
        "m.text": [{ "body": "Unbekannte FHIR-Datei wird nur zum Download angeboten" }]
      }]
    },
    "m.relates_to": {
      "event_id": "$1",
      "rel_type": "m.reference",
    },
  }
}
```

Bobs Client `B2` ist eine neuere Version und unterstützt das Ausfüllen von FHIR [`Questionnaire`]s
nativ. Er antwortet daher mit einer Erfolgsmeldung ohne Warnung.

``` json5
{
  "type": "de.gematik.msc4300.response.status",
  "sender": "@bob:bob.com",
  "content": {
    "m.response.status": {
      "from_device": "B2",
      "status": "success",
    },
    "m.relates_to": {
      "event_id": "$1",
      "rel_type": "m.reference",
    },
  }
}
```

Alices Client kann nun erkennen, dass beide von Bobs Geräten den Fragebogen empfangen haben und ein
Gerät in der Lage ist ihn auszufüllen. Sie kann daher sicher sein, dass Bob den Fragebogen
bearbeiten kann.

Bob füllt den Fragebogen auf seinem Gerät `B2` aus und sendet eine [`QuestionnaireResponse`] als
FHIR-Datei zurück. Um sicherzugehen, dass Alice, seine Antwort erhält und versteht, fragt er nun
seinerseits den Verarbeitungsstatus des Events an.

``` json5
{
  "event_id": "$2",
  "type": "m.room.message",
  "sender": "@bob:bob.com",
  "content": {
    // Ausgefüllter Fragebogen
    "msgtype": "m.file",
    "filename": "questionnaire_response.xml",
    "info": {
      "mimetype": "application/fhir+xml",
      ...
    },
    // Anfrage des Verarbeitungsstatus
    "de.gematik.msc4300.request.status": {
      "from_device": "B2",
      "lifetime": 600_000, // 10min
    }
    ...
  },
  ...
}
```

Alice Client empfängt den ausgefüllten Fragebogen, validiert den Inhalt und bietet ihn Alice zur
Weiterverarbeitung an bevor eine Erfolgsmeldung zurück gesendet wird.

``` json5
{
  "type": "de.gematik.msc4300.response.status",
  "sender": "@dr.alice:alice.com",
  "content": {
    "m.response.status": {
      "from_device": "A",
      "status": "success",
    },
    "m.relates_to": {
      "event_id": "$2",
      "rel_type": "m.reference",
    },
  }
}
```

Bob kann nun sicher sein, dass sein ausgefüllter Fragebogen empfangen und verstanden wurde.

## Sicherheit und Datenschutz

Die verpflichtende Antwort auf Abfragen zum Verarbeitungsstatus gibt den Online-Status des
Empfängers gegenüber dem Absender preis. Zur Mitigierung dieses Problems können Clients ihre Antwort
im Rahmen der `lifetime` von Anfragen verzögern.

In Räumen mit mehreren Teilnehmern wird der Online-Status nicht nur dem Absender sondern allen
Raumteilnehmern offengelegt. Dieses Problem erscheint vernachlässigbar, da der Regelfall in TI-M die
direkte Kommunikation sein wird. Zudem ließe sich der Verarbeitungsstatus wie in [MSC4300]
angedeutet in Zukunft auch per To-Device-Message kommunizieren.

## Kompatibilität & Migration

Clients, die dieses Proposal nicht implementieren werden Anfragen und Antworten zum
Verarbeitungsstatus ignorieren.

## Alternativen

Keine.

  [Spezifikation]: https://spec.matrix.org/v1.14/appendices/#common-namespaced-identifier-grammar
  [Dienstkennung]: https://fachportal.gematik.de/toolkit/dienstkennung-kim-kom-le
  [MSC4300]: https://github.com/matrix-org/matrix-spec-proposals/pull/4300
  [MSC4301]: https://github.com/matrix-org/matrix-spec-proposals/pull/4301
  [MSC4302]: https://github.com/matrix-org/matrix-spec-proposals/pull/4302
  [`Questionnaire`]: https://build.fhir.org/questionnaire-definitions.html
  [`QuestionnaireResponse`]: https://build.fhir.org/questionnaireresponse.html
