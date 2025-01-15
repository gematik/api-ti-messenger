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
entfallen. Diese Anforderungen sind der Vollständigkeit halber im nachfolgenden
Abschnitt "Aktuelle Löschanforderungen" aufgelistet. Des Weiteren sind die neuen
Anforderungen als grobe Vorschläge zu verstehen, die nicht zwingend mit
identischem Wortlaut in die Spezifikation übernommen werden sollen.

### Serverseitiges Löschen

**Story 1**

- Als Anbieter
- möchte ich nicht mehr benötigte Räume und deren Inhalte löschen
- damit ich die Speicherkosten meines Homeservers reduzieren kann.

**Story 2**

- Als Anbieter
- möchte ich meinen Nutzern verschiedene Preismodelle für die Haltung ihrer
  historischen Kommunikation anbieten
- damit ich meine Speicherkosten an sie weiterreichen kann.

**Story 3**

- Als Organisation im Gesundheitswesen
- möchte ich zentral vorgeben wann die Chats meiner Mitarbeiter gelöscht werden
- damit ich die Kontrolle über die Daten meiner Nutzer behalte.

Um diese Nutzungsszenarien zu realisieren ist es notwendig, dass Homeserver eine
konfigurierbare Funktion zum serverseitigen Löschen von Rauminhalten anbieten.
Da die Löschung in erster Linie der Wahrung der individuellen Wirtschaftlichkeit
und Datenhoheit dient, sind Voreinstellungen und Defaultwerte nicht erforderlich
sondern können dem Markt überlassen werden. Anbieter dürfen dabei im Rahmen
ihrer Preisgestaltung auch die Grenzen der Konfigurierbarkeit für Organisationen
festlegen.

Die Löschoperation selbst darf nur server-lokal und ohne direkte Auswirkung auf
die Föderation erfolgen, da ansonsten Löschkonfigurationen auf unterschiedlichen
Servern kollidieren könnten. Diese server-lokale Löschung schließt also z. B.
die Verwendung von Redactions oder das Kicken der Nutzer anderer Homeserver aus.

Für die konkrete technische Implementierung gibt es verschiedene Möglichkeiten.
So könnten z. B. Nutzer des eigenen Homeservers aus Räumen entfernt werden und
diese Räume samt der zugehörigen Events und Medien dann aus der Datenbank des
Servers gelöscht werden. Alternativ wäre auch ein fortlaufendes Löschen von
veralteten Events und Medien aus der Datenbank des Servers möglich wobei der
Raum selbst bestehen bleibt. Homeserver wie Synapse beispielsweise bieten
hierfür konfigurierbare [Message Retention Policies] an.

**A_1 - Serverseitige Löschfunktion**

Matrix-Homeserver MÜSSEN eine Funktion anbieten, durch die Rauminhalte und mit
einzelnen Räumen assoziierte Daten wie z. B. versendete Dateien nach einem
festgelegten Zeitraum seit letzter Aktivität im Raum gelöscht werden. Die
Löschung MUSS server-lokal und ohne direkten Einfluss auf die Föderation
erfolgen. **\[\<=\]**

Da sich, abgesehen von Raum-Avataren, Medien in Matrix aktuell nicht mit Räumen
verlinken lassen, impliziert die obige Formulierung die Verwendung von
[MSC3911].

### Clientseitiges Löschen

**Story 4**

- Als Endnutzer
- möchte ich selbst entscheiden wann meine Unterhaltungen gelöscht werden
- damit ich mir wichtige Inhalte nicht überraschend verliere.

**Story 5**

- Als Mitarbeiter im Gesundheitswesen
- möchte ich Chats, die ich in ein Drittsystem archiviert habe, löschen
- um Doppeldokumentation zu vermeiden.

**Story 6**

- Als Endnutzer
- möchte ich Chats löschen können
- damit meine Chatliste übersichtlich bleibt.

Die serverseitige Löschung von Rauminhalten propagiert allerspätestens dann wenn
der Nutzer das Gerät wechselt auch auf den Client. Der Nutzerwunsch über die
Löschung der eigenen Daten zu bestimmen kann daher mit dem Anbieter- und
Organisationswunsch Speicher zur Steigerung der Wirtschaftlichkeit freizugeben
kollidieren. Anbieter müssen ihre Nutzer deswegen über etwaige serverseitige
Löschungen informieren damit Daten nicht unerwartet verloren gehen. Die konkrete
Form der Information bleibt dabei dem Anbieter überlassen. So könnte der
Anbieter z. B. über [Server Notices] konkrete Räumlöschungen vorankündigen.
Alternativ wäre es auch denkbar, dass nur einmalig über feste Löschintervalle
informiert wird.

**A_2 - Information über serverseitige Löschung**

Anbieter MÜSSEN ihre Nutzer vorab über die serverseitige Löschung von Räumen und
Rauminhalten informieren. **\[\<=\]**

Weiterhin müssen Nutzer für die Organisation ihrer Unterhaltungen in die Lage
versetzt werden Räume selbstständig zu verlassen und vom Client zu entfernen.
Hierbei ist zu beachten, dass Räume nach erfolgreichem [`/leave`] weiterhin in
der [`/sync`]-Response auftauchen und dort erst durch ein explizites [`/forget`]
verschwinden. Clients steht es frei diese beiden Operationen zu kombinieren oder
zu trennen. Werden sie getrennt, entsteht dadurch gewissermaßen eine
Zwischenablage für historische Räume.

**A_3 - Verlassen von Räumen**

Clients MÜSSEN Nutzern erlauben Räume über die Nutzung der APIs [`/leave`] und
[`/forget`] vom Client zu löschen. **\[\<=\]**

*Hinweis: Clients steht es frei die Operationen [`/leave`] und [`/forget`] zu
trennen um dem Nutzer zu ermöglichen ein Archiv verlassener Räume zu erstellen.*

Eine regelmäßige Erinnerung an das clientseitige Löschen von Räumen ist nicht
erforderlich. Zum Einen ist der überwiegende Teil der Kommunikation in TI-M
Ende-zu-Ende verschlüsselt. Zum Anderen lässt sich bei mehreren Raumteilnehmern
von verschiedenen Homeservern vom Client ohnehin kein tatsächliches Löschen der
Daten auf dem Server durchsetzen. Gleichzeitig steht es Herstellern aber frei
intelligente Systeme zur Erkennung abgeschlossener Unterhaltungen zu entwickeln
und diese dem Nutzer zur Löschung anzubieten.

### Redactions

**Story 7**

- Als Endnutzer
- möchte ich von mir gesendete Nachrichten für alle Gesprächsteilnehmer löschen
- damit ich schwerwiegende Fehler korrigieren kann.

Die Matrix-Spezifikation ermöglicht die (Selbst-)Moderation von Events mittels
[Redactions]. Redactions sind eine invasive Form des Löschens da sie über die
Föderation propagieren und letztendlich zu einer irreversiblen Löschung von
Inhalten auf allen beteiligten Servern und Clients führen.

Dieses Verhalten ist in bestimmten Fällen wünschenswert. Man stelle sich z. B.
vor es wurden Daten eines Patienten versehentlich an einen anderen Patienten
gesendet. Hier erscheint es sinnvoller den Versender diesen Fehler direkt per
Redaction korrigieren zu lassen anstatt einen langsameren Eskalationsprozess mit
Löschung durch einen Administrator zu verwenden.

Gleichzeitig führen Redactions bei Fehlbenutzung aber zu einem unerwarteten
Verlusts von eigentlich relevanten Nachrichten für andere Gesprächsteilnehmer.
Als Kompromiss werden Redactions daher zwar erlaubt. Sie müssen im Client aber
stets mit einem Warnhinweis versehen werden[^1].

**A_4 - Nachrichtenbasiertes Löschen per Redaction**

Clients MÜSSEN ihren Nutzern erlauben eigene Nachrichten per Redaction zu
löschen. Dabei MUSS der Nutzer vor jedem Auslösen einer Redaction per
Warnhinweis darauf hingewiesen werden, dass die Nachricht irreversibel und für
alle Gesprächsteilnehmer gelöscht wird. **\[\<=\]**

**A_5 - Kennzeichnung gelöschter Nachrichten**

Clients MÜSSEN `m.room.redaction` Events analog zu Servern anwenden und
gelöschte Nachrichten mit Datum, Uhrzeit und löschendem Akteur kennzeichnen.
**\[\<=\]**

Unabhängig von Redactions können TI-M Clients bei Bedarf visuelles Löschen für
z. B. `m.room.message` Events auch über [Event Replacements] implementieren.
Diese Form des Löschens ist reversibel und transparent da Replacements separate
Events sind und die gesamte Historie von Events erhalten bleibt.

### DSGVO & Datenschutz

**Story 8**

- Als Endnutzer
- möchte ich mein Recht auf Löschung nach DSGVO wahrnehmen
- damit ich die Kontrolle über meine Daten behalte.

TI-M Clients und Fachdienste müssen DSGVO konform sein. Dies gilt ganz
unabhängig von der TI-M Spezifikation und Bedarf an dieser Stelle eigentlich
keiner weiteren Regelung. Insbesondere kann das Recht auf Löschung der eigenen
Daten auch durch einen Supportprozess außerhalb des Clients implementiert
werden.

Da die Matrix-Spezifikation zwischen [`/leave`] und [`/forget`] unterscheidet
und dies nicht selten zu Verwirrung führt erscheinen einige explizite Regelungen
dennoch sinnvoll.

Zum einen müssen private Räume, die von allen Teilnehmern eines Homeservers
verlassen und per [`/forget`] clientseitig entfernt wurden auch serverseitig
gelöscht werden. Dies folgt direkt aus dem DSGVO-Prinzip der Datensparsamkeit
und der Tatsache, dass Nutzer diese Räume nicht mehr betreten können und auch
auf ihren Geräten zur Löschung freigegeben haben.

Zum anderen dürfen private Räume, die zwar von allen Teilnehmern eines
Homeservers verlassen aber nicht per [`/forget`] zur Löschung freigegeben
wurden, erst nach einer gewissen Karenzzeit auf dem Homeserver gelöscht werden.
Dies ergibt sich insbesondere daraus, dass das Verlassen eines Raumes nach
[A_26348] und [A_26463] nicht zwingend vom Nutzer selbst initiert wird. Die
Karenzzeit dient daher dazu die Wahrscheinlichkeit eines unerwarteten Verlustes
von Rauminhalten für den Nutzer zu reduzieren.

**A_6 - Serverseitiges Löschen nach `/forget`**

Homeserver MÜSSEN einen Raum und dessen Inhalte lokal löschen, wenn:

- der Raum privat ist und
- keiner der Nutzer des Homservers im Raum die Membership `invite` oder `join`
  hat und
- alle Nutzer des Homeservers, deren Membership im Raum `leave` oder `ban` ist,
  den Raum per [`/forget`] von ihren Clients entfernt haben.

Diese Löschung MUSS innerhalb von 7 Tagen ab letztem [`/forget`] erfolgen.
**\[\<=\]**

**A_7 - Serverseitiges Löschen ohne `/forget`**

Homeserver MÜSSEN einen Raum und dessen Inhalte für mindestens 7 Tage vorhalten,
wenn:

- der Raum privat ist und
- keiner der Nutzer des Homservers im Raum die Membership `invite` oder `join`
  hat und
- es Nutzer des Homeservers gibt, deren Membership im Raum `leave` oder `ban`
  ist, die den Raum aber *nicht* per [`/forget`] von ihren Clients entfernt
  haben.

Nach Ablauf dieser Karenzzeit, ist eine serverseite Löschung erlaubt.
**\[\<=\]**

## Aktuelle Löschanforderungen

Da das aktuelle Löschkonzept wie eingangs erwähnt fragmentiert ist, werden der
Vollständigkeit halber im Folgenden die momentan existierenden Anforderungen aus
[TI-M Basis] aufgelistet. Die Anforderungen sind dabei zum besseren Verständnis
in dieselben Kategorien wie im vorigen Abschnitt "Änderungsvorschlag" gruppiert.

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

[^1]: Dieses Verhalten wird in ähnlicher Form auch bei der Löschung von Dokument
    aus der ePA verwendet. Siehe hierzu [A_20103].

[^2]: Der zweite Teil dieser Anforderung ist redundant zu [A_25575] und gehört
    daher eigentlich auch in die Kategorie "Redactions".

  [TI-M Basis]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1
  [Message Retention Policies]: https://element-hq.github.io/synapse/latest/message_retention_policies.html
  [MSC3911]: https://github.com/matrix-org/matrix-spec-proposals/pull/3911
  [Server Notices]: https://spec.matrix.org/v1.13/client-server-api/#server-notices
  [`/leave`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3roomsroomidleave
  [`/sync`]: https://spec.matrix.org/v1.13/client-server-api/#get_matrixclientv3sync
  [`/forget`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3roomsroomidforget
  [Redactions]: https://spec.matrix.org/v1.13/client-server-api/#redactions
  [Event Replacements]: https://spec.matrix.org/v1.13/client-server-api/#event-replacements
  [A_26348]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_ePA/latest/#A_26348
  [A_26463]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Pro/latest/#A_26463
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
