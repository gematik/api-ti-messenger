# Löschkonzept v2

Das aktuelle Konzept zum Löschen von Räumen und Inhalten in [TI-M Basis] wird in
Kommentierungsworkshops und Supportanfragen regelmäßig hinterfragt und
kritisiert. Dabei fallen insbesondere die folgenden Probleme auf:

- Das Konzept ist fragmentiert. Anforderungen tauchen in verschiedenen Kapiteln
  und ohne übergreifende Erläuterung auf. Das Konzept und dessen Motivation
  holistisch zu verstehen ist daher sehr schwierig.
- Die einzelnen Anforderungen sind teilweise überlappend oder lassen zu viel
  Spielraum für Interpretationen. Beispielsweise sind etwaige Wechselwirkung
  zwischen client- und serverseitigem Löschen oder die Auswirkungen des Löschens
  auf die Föderation nicht klar benannt. Unterschiedliche Interpretationen
  können zu unterschiedlichen Implementierungen und im schlimmsten Fall zu
  Inkompatibilitäten führen.
- Die Anforderungen entsprechen teilweise nicht den Bedürfnissen und Erwartungen
  von Nutzern, Organisationen und Anbietern. Rückmeldungen aus der
  TI-Modellregion Hamburg z. B. haben ergeben, dass Nutzer nicht verstehen warum
  sie regelmäßig gefragt werden ob sie Räume, die älter als 6 Monate sind
  löschen möchten und ob diese Löschung Auswirkungen auf andere Raumteilnehmer
  hat.

Im hier vorliegenden Proposal wird ausgehend von User Stories ein Vorschlag für
ein verständliches, sinnvolles und begründetes Löschkonzept präsentiert, das
obige Probleme löst.

## Änderungsvorschlag

Zur Erarbeitung eines Änderungsvorschlags wurden zunächst die Bedürfnisse von
Nutzern, Organisationen und Anbietern durch gezielte Nachfragen bei
Industriepartnern zusammengetragen. Aus diesen Bedürfnissen wurden im nächsten
Schritt User Stories abgeleitet und gruppiert. Im Folgenden werden die
gruppierten User Stories und die daraus abgeleiteten Änderungsvorschläge
vorgestellt.

Ausgangspunkt der Vorschläge ist, dass alle aktuellen Anforderung zum Löschen
entfallen. Diese Anforderungen sind der Vollständigkeit halber weiter unten im
Abschnitt "Aktuelle Löschanforderungen" aufgelistet. Des Weiteren sind die neuen
Anforderungen als unverbindliche Vorschläge zu verstehen, die nicht zwingend mit
identischem Wortlaut in die Spezifikation übernommen werden sollen.

Zum besseren Verständnis der weiteren Ausführungen ist im Folgenden der Verlauf
einer TI-M Nachricht von einem Mitarbeiter des Gesundheitswesens bis zu einem
Versicherten dargestellt.

![TI-M Architektur]

### Serverseitiges Löschen

**Story 1**

- Als Versicherter
- möchte ich selbst entscheiden wann meine Unterhaltungen gelöscht werden
- damit ich mir wichtige Inhalte nicht überraschend verliere.

**Story 2**

- Als Anbieter
- möchte ich nicht mehr benötigte Räume und deren Inhalte (v. a. Medien) löschen
- damit ich die Speicherkosten meines Homeservers reduzieren kann.

**Story 3**

- Als Anbieter
- möchte ich Leistungserbringern verschiedene Preismodelle für die Haltung ihrer
  historischen Kommunikation anbieten
- damit ich meine Speicherkosten an sie weiterreichen kann.

**Story 4**

- Als Organisation im Gesundheitswesen
- möchte ich zentral vorgeben wann die Chats meiner Mitarbeiter gelöscht werden
- damit ich die Kontrolle über die Daten meiner Nutzer behalte.

Aus diesen Stories wird ersichtlich, dass es eine Unterscheidung geben muss
zwischen dem Löschen für Versicherte (TI-M ePA) und dem Löschen für Mitarbeiter
des Gesundheitswesens (TI-M Pro). Da diese beiden Nutzergruppen verschiedene
Clients und Fachdienste verwenden, lässt sich diese Unterscheidung auch gut
technisch abbilden.

Versicherte auf der einen Seite haben grundsätzlich die Hoheit über ihre Daten
und Kommunikation. Im Gegensatz zu Mitarbeitern im Gesundheitswesen ist
anzunehmen, dass Versicherte zudem kein Archivsystem haben, in das sie TI-M
Inhalte exportieren können. Hier darf es daher keine automatische Löschung ohne
vorherige Bestätigung durch den Versicherten geben, da sonst Inhalte unerwartet
verloren gehen würden.

Hier darf es daher keine automatische Löschung ohne Nutzereinwilligung geben da
sonst Inhalte unerwartet und unwiederbringlich verloren gehen könnten.

**A_1 - Serverseitige Löschung nur nach Nutzerbestätigung**

TI-M ePA Fachdienste DÜRFEN Rauminhalte (= Events) NICHT ohne vorige Bestätigung
aller lokalen Teilnehmer des Raumes löschen. Redactions sind von dieser Regelung
ausgenommen. **\[\<=\]**

Mitarbeiter des Gesundheitswesens auf der anderen Seite sind ihren
Organisationen und deren Regeln untergeordnet. Für diese Organisationen wiederum
gelten gesetzliche Vorgaben zur Datenhaltung, die eine automatische Löschung
nötig machen können. Beispiele hierfür sind [DSGVO Art. 17] und [SGB 5 § 304].

Die Löschoperation selbst darf dabei allerdings nur server-lokal und ohne
direkte Auswirkung auf die Föderation erfolgen damit Löschkonfigurationen auf
unterschiedlichen Servern nicht interferieren. Diese server-lokale Löschung
schließt z. B. die Verwendung von Redactions oder das Kicken der Nutzer anderer
Homeserver aus.

**A_2 - Automatische serverseitige Löschung**

TI-M Pro Fachdienste KÖNNEN Rauminhalte (= Events) automatisch und ohne
Einwilligung der Raumteilnehmer löschen. **\[\<=\]**

**A_3 - Lokale Beschränkung der automatischen serverseitigen Löschung**

Implementieren TI-M Pro Fachdienste Funktionen zum automatisch Löschen von
Rauminhalten (= Events), so MUSS die Löschung server-lokal und ohne direkten
Einfluss auf die Föderation erfolgen. **\[\<=\]**

Damit Inhalte nicht unerwartet verloren gehen, müssen Nutzer über etwaige
automatische Löschfunktionen zumindest informiert werden. Die konkrete Form der
Information bleibt dabei dem Anbieter überlassen. So könnte der Anbieter z. B.
über [Server Notices] einzelne Räumlöschungen vorankündigen. Alternativ wäre es
auch denkbar, dass nur einmalig über feste Löschintervalle informiert wird.

**A_4 - Information über automatische serverseitige Löschung**

Nutzen TI-M Pro Anbieter Funktionen zur automatischen serverseitigen Löschung
von Rauminhalten (= Events), so MÜSSEN sie ihre Nutzer vorab darüber
informieren. **\[\<=\]**

Die konkrete Ausgestaltung einer automatischen serverseitigen Löschfunktion muss
durch die Spezifikation nicht weiter vorgegeben werden. Hier gibt es
verschiedene Möglichkeiten. So könnten z. B. Nutzer des eigenen Homeservers aus
Räumen entfernt werden und diese Räume samt der zugehörigen Events dann aus der
Datenbank des Servers gelöscht werden. Alternativ wäre auch ein fortlaufendes
Löschen von veralteten Events aus der Datenbank des Servers möglich, wobei der
Raum selbst bestehen bleibt. Homeserver wie Synapse bieten hierfür
konfigurierbare [Message Retention Policies] an. Eine weitere Möglichkeit wäre
die Implementierung eines Quota-Systems wie es z. B. bei E-Mail-Anbietern gängig
ist.

#### Serverseitiges Löschen von Medien

Die vorangegangenen Überlegungen für Events sollten prinzipiell auch für Medien
gelten. Hierbei gibt es aber zwei technologische Einschränkungen:

1.  Events lassen sich aktuell in Matrix nur im unverschlüsselten Zustand, also
    nur auf Clients, mit Medien verknüpfen (vgl. auch obige Architekturskizze).
2.  Es gibt in Matrix momentan keine API zum Löschen von Medien, die ein Client
    benutzen könnte.

Für beide Probleme existieren Lösungsvorschläge ([MSC3911] bzw. [MSC2278]).
Diese stellen allerdings weitreichende und bisher nicht als praktikabel
bewiesene Eingriffe ins Protokoll dar. Die Lösung dieser Probleme wird daher
hier aus dem Scope ausgeklammert und auf eine zukünftige Iteration des
Löschkonzepts verschoben.

Geht man also vom aktuellen Zustand aus, so haben Homeserver abgesehen vom
Zeitpunkt des letzten Downloads, keine Information darüber ob Medien obsolet
sind oder nicht. Gleichzeitig haben Clients keine Möglichkeit Medien selbst zu
löschen oder als obsolet zu kennzeichnen. Eine dauerhafte und anlasslose
Speicherung von Daten, egal ob verschlüsselt oder nicht, ist nach DSGVO aber
nicht zulässig. Hieraus resultiert zwangsläufig, dass Homeserver Medien nach
Ablauf eines Intervalls automatisch Löschen müssen. Die konkrete Festlegung des
Intervalls obliegt dabei dem Betreiber.

**A_5 - Automatische Löschung von Medien**

TI-M Fachdienste müssen Medien nach Ablauf eines konfigurierbaren Intervalls
seit Empfang oder letztem Download löschen. **\[\<=\]**

Es sei an dieser Stelle betont, dass diese automatische Löschung von Medien, die
ganz offensichtlich den zuvor benannten Zielen für Events widerspricht, nur eine
Zwischenlösung ist.

### Clientseitiges Löschen

**Story 5**

- Als Endnutzer
- möchte ich nicht mehr benötigte Chats löschen können
- um Ordnung in meiner Chatliste zu schaffen.

**Story 6**

- Als Mitarbeiter im Gesundheitswesen
- möchte ich Chats, die ich in ein Drittsystem archiviert habe, löschen
- um Doppeldokumentation zu vermeiden.

Nutzer müssen für die Organisation ihrer Unterhaltungen in die Lage versetzt
werden Räume selbstständig verlassen und vom Client entfernen zu können. Hierbei
ist zu beachten, dass Matrix zwischen [`/leave`] und [`/forget`] unterscheidet.
Ein Nutzer nimmt nach [`/leave`] nicht mehr an der weiteren Kommunikation in
einem Raum teil. Er kann die bis dahin gesendeten Inhalte aber weiterhin
abrufen. Erst nach [`/forget`] hat der Nutzer keinen Zugriff mehr auf die
Raumhistorie.

Clients steht es frei nach dem Verlassen eines Raumes durch [`/leave`]
automatisch auch ein Vergessen per [`/forget`] auszuführen. Tun sie das nicht,
ermöglichen sie ihren Nutzern damit die Verwaltung einer Zwischenablage für
historische Räume.

Da das Verlassen eines Raumes auch fremdausgelöst sein kann, z. B. durch
[A_26348], müssen Clients historische Räume aber in jedem Falle in ihrem UI
zugänglich machen. Hierfür kann ein [Filter] mit `include_leave = true`
verwendet werden wodurch alle verlassenen aber noch nicht vergessenen Räume mit
dem Initial [`/sync`] zurückgeliefert werden. Dabei muss allerdings beachtet
werden, dass es in Matrix aktuell keinen Mechanismus zum Synchronisieren von
vergessenen Räumen über mehrere Geräte eines Nutzers hinweg gibt. Sofern die
gleichzeitige Anmeldung auf mehreren Geräten nicht technisch ausgeschlossen
wird, empfiehlt es sich daher in gewissen Abständen einen Initial [`/sync`]
auszuführen um neu vergessene Räume einzusammeln.

**A_6 - Vergessen von Räumen**

TI-M Clients MÜSSEN Nutzern erlauben Räume über die Nutzung der APIs [`/leave`]
und [`/forget`] vom Client zu löschen. Dabei können Clients diese Operationen
wahlweise getrennt oder nur kombiniert auslösbar machen. **\[\<=\]**

**A_7 - Anzeige historischer Räume**

TI-M Clients MÜSSEN Räume, die verlassen aber noch nicht mittels [`/forget`]
vergessen wurden, per [`/sync`] abfragen und in ihrem UI zugänglich machen.
**\[\<=\]**

Damit Clients tatsächlich die Wahl haben ob sie [`/forget`] automatisch
ausführen oder nicht dürfen Homeserver diesen Automatismus nicht selbst
implementieren (wie es z. B. bei der [`forget_rooms_on_leave`] Konfiguration in
Synapse passiert).

**A_8 - Keine serverseitige Kombination von `/leave` und `/forget`**

TI-M Fachdienste DÜRFEN bei Aufruf der API [`/leave`] NICHT automatisch ein
[`/forget`] ausführen. **\[\<=\]**

Haben alle Teilnehmer eines Homeservers einen privaten Raum verlassen und per
[`/forget`] clientseitig entfernt, so muss dieser Raum mit seinen Inhalten auch
serverseitig gelöscht werden. Dies folgt direkt aus dem DSGVO-Prinzip der
Datensparsamkeit und der Tatsache, dass Nutzer diese Räume nicht mehr betreten
können und auch auf ihren Geräten zur Löschung freigegeben haben.

**A_9 - Serverseitiges Löschen nach `/forget`**

TI-M Fachdienste MÜSSEN einen Raum und dessen Inhalte lokal löschen, wenn:

- der Raum privat ist (im Sinne von Join Rules und History Visibility) und
- keiner der Nutzer des Homservers im Raum die Membership `invite` oder `join`
  hat und
- alle Nutzer des Homeservers, deren Membership im Raum `leave` oder `ban` ist,
  den Raum per [`/forget`] von ihren Clients entfernt haben.

Diese Löschung MUSS innerhalb von 7 Tagen ab letztem [`/forget`] erfolgen.
**\[\<=\]**

Die Regelungen zum serverseitigen Löschen bei [`/leave`] *ohne* [`/forget`]
richten sich nach dem vorigen Abschnitt.

### Redactions

**Story 7**

- Als Endnutzer
- möchte ich von mir gesendete Nachrichten für alle Gesprächsteilnehmer löschen
- damit ich schwerwiegende Fehler korrigieren kann.

Die Matrix-Spezifikation ermöglicht die Selbstmoderation von Events mittels
[Redactions]. Redactions sind eine invasive Form des Löschens da sie über die
Föderation propagieren und letztendlich zu einer irreversiblen Löschung von
Inhalten auf allen beteiligten Servern und Clients führen.

Dieses Verhalten ist in bestimmten Fällen wünschenswert. Man stelle sich z. B.
vor es wurden Daten eines Patienten versehentlich an einen anderen Patienten
gesendet. Hier erscheint es sinnvoller den Versender diesen Fehler direkt per
Redaction korrigieren zu lassen anstatt einen langsameren Eskalationsprozess mit
Löschung durch einen Administrator zu verwenden.

Gleichzeitig führen Redactions bei Fehlbenutzung aber zu einem unerwarteten
Verlust von eigentlich relevanten Nachrichten für andere Gesprächsteilnehmer.
Als Kompromiss werden Redactions daher zwar erlaubt. Sie werden aber zeitlich
eingeschränkt und müssen im Client stets mit einem Warnhinweis versehen
werden[^1].

**A_10 - Nachrichtenbasiertes Löschen per Redaction**

Clients MÜSSEN ihren Nutzern erlauben eigene Nachrichten per Redaction innerhalb
von 24h ab `origin_server_ts` zu löschen. Dabei MUSS der Nutzer vor jedem
Auslösen einer Löschung per Warnhinweis darauf hingewiesen werden, dass die
Nachricht irreversibel und für alle Gesprächsteilnehmer gelöscht wird.

Ist eine zu löschende Nachricht Ausgangspunkt einer Kette von [Event
Replacements], so MÜSSEN alle Events dieser Kette redacted werden. Dies kann
z.B. über mehrere einzelne Redactions oder einen Mechanismus wie in [MSC3912]
geschehen. **\[\<=\]**

**A_11 - Zeitgrenze für Redactions**

Fachdienste MÜSSEN Redactions der eigenen Nachrichten eines Nutzers ablehnen
wenn seit `origin_server_ts` des zu redactenden Events mehr als 24h vergangen
sind. **\[\<=\]**

**A_12 - Kennzeichnung gelöschter Nachrichten**

Clients MÜSSEN `m.room.redaction` Events analog zu Servern anwenden und
gelöschte Nachrichten mit Datum, Uhrzeit und löschendem Akteur kennzeichnen.
**\[\<=\]**

Unabhängig von Redactions können TI-M Clients bei Bedarf visuelles Löschen für
z. B. `m.room.message` Events auch über [Event Replacements] implementieren.
Diese Form des Löschens ist reversibel und transparent da Replacements separate
Events sind und die gesamte Historie von Events erhalten bleibt.

Redactions können weiterhin auch zur Fremdmoderation, also dem Löschen von
Events *anderer* Nutzer, verwendet werden, z. B. in Verbindung mit dem Modul
[Reporting Content]. Im TI-M Kontext stellt dies momentan allerdings keinen
nennenswerten Use Case dar. Das liegt zum einen daran, dass alle Nutzer-Accounts
gemanagt und identifizierbar sind. Der Versand von z. B. illegalen Inhalten
würde daher in erster Linie den Absender und nicht den Empfänger belasten. Zum
anderen ist Fremdmoderation in der Regel nur in öffentlichen und
unverschlüsselten Räumen sinnvoll und praktikabel. Solche Räume können aktuell
aber nur lokal auf TI-M Pro Homeservern existieren. Das Thema der
Fremdmoderation wird daher an dieser Stelle nicht weiter behandelt.

## Bisherige und in Zukunft entfallende Löschanforderungen

Da das aktuelle Löschkonzept wie eingangs erwähnt fragmentiert ist, werden der
Vollständigkeit halber im Folgenden die momentan existierenden Anforderungen aus
[TI-M Basis] aufgelistet. Die Anforderungen sind dabei zum besseren Verständnis
in dieselben Kategorien wie im vorigen Abschnitt "Änderungsvorschlag" gruppiert.
Wie zuvor erwähnt, entfallen alle bisherigen Anforderungen und werden durch die
im Änderungsvorschlag aufgelisteten neuen Anforderungen ersetzt.

### Serverseitiges Löschen

> **[A_25318] - Löschfunktion für Matrix-Homeserver**
>
> Matrix-Homeserver MÜSSEN eine Funktion anbieten, durch die Events,
> Gesprächsinhalte und mit einzelnen Gesprächen assoziierte Daten (z. B.
> versandte Dateien) nach einem festgelegten Zeitraum seit letzter Aktivität in
> einem Raum gelöscht werden. **\[\<=\]**

> **[A_25319] - Zeitraum zur Auslösung der Löschfunktion für Matrix-Homeserver**
>
> Die Löschfunktion für Matrix-Homeserver MUSS auf sechs Monate voreingestellt
> und durch einen Akteur in der Rolle "Org-Admin" konfigurierbar sein.
> **\[\<=\]**
>
> Hinweis: Die Löschfunktion für Matrix-Homeserver kann derart realisiert
> werden, dass nach Verstreichen des festgelegten Zeitraums die Teilnehmer einen
> Gesprächsraum verlassen - bspw. indem sie vom Homeserver aus diesem entfernt
> werden - und der Raum nach Verlassen aller Teilnehmer automatisch gelöscht
> wird.
>
> Hinweis: Die Löschfunktion für Matrix-Homeserver kann per Opt-Out durch den
> Akteur in der Rolle "Org-Admin" deaktivierbar sein.

### Clientseitiges Löschen

> **[A_25609] - Konfigurierbare Frist zur Erinnerung an Löschung**
>
> TI-M Clients MÜSSEN über eine konfigurierbare Frist zur Erinnerung an die
> Löschung von Räumen und Rauminhalten verfügen, welche die Löschung aller
> Daten, die älter als die konfigurierte Frist sind, anbietet. **\[\<=\]**

> **[A_25610] - Voreinstellung zur Erinnerung an Löschung**
>
> Die konfigurierbare Frist in TI-M Clients zur Erinnerung an die Löschung von
> Räumen und Rauminhalten MUSS auf sechs Monate voreingestellt sein und bezieht
> sich auf den Zeitstempel der Erstellung der letzten Nachricht in einem Raum.
> **\[\<=\]**

> **[A_25611] - Durchführung der Löschung**
>
> Ist die im TI-M Client konfigurierte Frist zur Erinnerung an die Löschung von
> Räumen und Rauminhalten verstrichen, so MUSS der Client den Nutzer darauf
> hinweisen und die Löschung des Raumes und dessen Inhalten empfehlen. Stimmt
> der Nutzer zu, so wird er aus dem Raum entfernt, was auch zu einer
> Benachrichtigung des Servers führt. **\[\<=\]**

> **[A_25492] - Löschfunktion des Clients[^2]**
>
> Der TI-M Client MUSS Löschfunktionen implementieren, die es dem Akteur
> ermöglichen, Daten sowohl lokal auf dem Client zu löschen, bspw. durch
> Verlassen eines Raumes, bei dem der gesamte Raum samt Inhalt vom Client
> entfernt wird, als auch server-seitig, wenn es sich dabei um Daten handelt,
> die der Nutzer selbst eingebracht hat, bspw. indem er eine von ihm verfasste
> Nachricht in einem Chat-Raum löscht ("redact"). **\[\<=\]**

### Redactions

> **[A_25575] - Löschung von Nachrichten**
>
> TI-M Clients MÜSSEN über eine nachrichtenbasierte Löschfunktion verfügen, die
> es Akteuren erlaubt, ihre eigenen Nachrichten gemäß \[Client-Server
> API/#redactions\] zu löschen. **\[\<=\]**

> **[A_25576] - Lokales Löschen bei Entfernung von Nachrichten aus dem Room
> State**
>
> Wurde eine Nachricht durch einen anderen Akteur gemäß \[Client-Server
> API/#redactions\] gelöscht, so MÜSSEN TI-M Clients, die sich im selben Raum
> befinden, die entsprechende Nachricht auch lokal löschen und kennzeichnen.
> **\[\<=\]**

> **[A_25577] - Kennzeichnung gelöschter Nachrichten**
>
> TI-M Clients MÜSSEN im Rahmen der Kennzeichnung gelöschter Nachrichten den
> löschenden Akteur, das Datum und die Uhrzeit der Löschung enthalten.
> **\[\<=\]**

[^1]: Dieses Verhalten wird in ähnlicher Form auch bei der Löschung von
    Dokumenten aus der ePA verwendet. Siehe hierzu [A_20103].

[^2]: Der zweite Teil dieser Anforderung ist redundant zu [A_25575] und gehört
    daher eigentlich auch in die Kategorie "Redactions".

  [TI-M Basis]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1
  [TI-M Architektur]: 291-architektur.png "Architektur"
  [DSGVO Art. 17]: https://dsgvo-gesetz.de/art-17-dsgvo/
  [SGB 5 § 304]: https://www.gesetze-im-internet.de/sgb_5/__304.html
  [Server Notices]: https://spec.matrix.org/v1.13/client-server-api/#server-notices
  [Message Retention Policies]: https://element-hq.github.io/synapse/latest/message_retention_policies.html
  [MSC3911]: https://github.com/matrix-org/matrix-spec-proposals/pull/3911
  [MSC2278]: https://github.com/matrix-org/matrix-spec-proposals/pull/2278
  [`/leave`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3roomsroomidleave
  [`/forget`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3roomsroomidforget
  [A_26348]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_ePA/latest/#A_26348
  [Filter]: https://spec.matrix.org/v1.13/client-server-api/#filtering
  [`/sync`]: https://spec.matrix.org/v1.13/client-server-api/#get_matrixclientv3sync
  [`forget_rooms_on_leave`]: https://element-hq.github.io/synapse/latest/usage/configuration/config_documentation.html#forget_rooms_on_leave
  [Redactions]: https://spec.matrix.org/v1.13/client-server-api/#redactions
  [Event Replacements]: https://spec.matrix.org/v1.13/client-server-api/#event-replacements
  [MSC3912]: https://github.com/matrix-org/matrix-spec-proposals/pull/3912
  [Reporting Content]: https://spec.matrix.org/v1.13/client-server-api/#reporting-content
  [A_25318]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25318
  [A_25319]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25319
  [A_25609]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25609
  [A_25610]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25610
  [A_25611]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25611
  [A_25492]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25492
  [A_25575]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25575
  [A_25576]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25576
  [A_25577]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25577
  [A_20103]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_ePA_FdV/latest/#A_20103
