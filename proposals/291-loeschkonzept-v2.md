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

Zum besseren Verständnis der weiteren Ausführungen ist im Folgenden der Verlauf
einer TI-M Nachricht von einem Versicherten bis zu einem Mitarbeiter des
Gesundheitswesens dargestellt.

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

Versicherte auf der einen Seite haben die alleinige Hoheit über ihre Daten und
Kommunikation. Im Gegensatz zu Mitarbeitern im Gesundheitswesen haben
Versicherte zudem kein Archivsystem, in das sie TI-M Inhalte exportieren können.
Hier darf es daher keine automatische Löschung ohne Nutzereinwilligung geben da
sonst Inhalte unerwartet und unwiederbringlich verloren gehen könnten.

**A_1 - Serverseitige Löschung nur nach Nutzereinwilligung**

TI-M ePA Fachdienste DÜRFEN Rauminhalte und damit verbundene Medien NICHT ohne
vorige Einwilligung aller lokalen Teilnehmer des Raumes löschen. Redactions sind
von dieser Regelung ausgenommen. **\[\<=\]**

Mitarbeiter des Gesundheitswesens auf der anderen Seite sind ihrer Organisation
und deren Regeln untergeordnet. Darüber hinaus gelten für sie gesetzliche
Vorgaben zur Datenhaltung, die eine automatische Löschung nötig machen können.
Beispiele hierfür sind [DSGVO Art. 17] und [SGB 5 § 304].

Die Löschoperation selbst darf dabei allerdings nur server-lokal und ohne
direkte Auswirkung auf die Föderation erfolgen damit Löschkonfigurationen auf
unterschiedlichen Servern nicht interferieren. Diese server-lokale Löschung
schließt z. B. die Verwendung von Redactions oder das Kicken der Nutzer anderer
Homeserver aus.

**A_2 - Automatische serverseitige Löschung**

TI-M Pro Fachdienste KÖNNEN Rauminhalte und damit verbundene Medien automatisch
und ohne Einwilligung der Raumteilnehmer löschen. **\[\<=\]**

**A_3 - Lokale Beschränkung der automatischen serverseitigen Löschung**

Implementieren TI-M Pro Fachdienste Funktionen zum automatisch Löschen von
Rauminhalten oder damit verbundener Medien, so MUSS die Löschung server-lokal
und ohne direkten Einfluss auf die Föderation erfolgen. **\[\<=\]**

Damit Inhalte nicht unerwartet verloren gehen, müssen Nutzer über etwaige
automatische Löschfunktionen zumindest informiert werden. Die konkrete Form der
Information bleibt dabei dem Anbieter überlassen. So könnte der Anbieter z. B.
über [Server Notices] einzelne Räumlöschungen vorankündigen. Alternativ wäre es
auch denkbar, dass nur einmalig über feste Löschintervalle informiert wird.

**A_4 - Information über automatische serverseitige Löschung**

Nutzen TI-M Pro Anbieter Funktionen zur automatischen serverseitigen Löschung
von Rauminhalte oder damit verbundener Medien, so MÜSSEN sie ihre Nutzer vorab
darüber informieren. **\[\<=\]**

Die konkrete Ausgestaltung einer automatischen serverseitigen Löschfunktion muss
durch die Spezifikation nicht weiter vorgegeben werden. Hier gibt es
verschiedene Möglichkeiten. So könnten z. B. Nutzer des eigenen Homeservers aus
Räumen entfernt werden und diese Räume samt der zugehörigen Events und Medien
dann aus der Datenbank des Servers gelöscht werden. Alternativ wäre auch ein
fortlaufendes Löschen von veralteten Events und Medien aus der Datenbank des
Servers möglich, wobei der Raum selbst bestehen bleibt. Homeserver wie Synapse
bieten hierfür konfigurierbare [Message Retention Policies] an. Eine weitere
Möglichkeit wäre die Implementierung eines Quota-Systems wie es z. B. bei
E-Mail-Anbietern gängig ist. Die hierfür eventuell notwendige Verlinkung von
Räumen und Medien könnte beispielsweise durch [MSC3911] realisiert werden.

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
ist zu beachten, dass Räume nach erfolgreichem [`/leave`] weiterhin per
[`/sync`] abrufbar sind. Dafür müssen Clients lediglich einen entsprechenden
[Filter] mit `include_leave = true` verwenden. Erst durch ein explizites
[`/forget`] verschwinden verlassene Räume dauerhaft aus der [`/sync`]-Response.

Clients steht es frei die Operationen [`/leave`] und [`/forget`] zu kombinieren
oder zu trennen. Werden sie getrennt, entsteht dadurch gewissermaßen eine
Zwischenablage für historische Räume. Um diese Trennung zu ermöglichen dürfen
Homeserver die beiden Operationen nicht automatisch kombinieren (wie z. B. bei
der [`forget_rooms_on_leave`] Konfiguration in Synapse).

**A_5 - Verlassen von Räumen**

TI-M Clients MÜSSEN Nutzern erlauben Räume über die Nutzung der APIs [`/leave`]
und [`/forget`] vom Client zu löschen. **\[\<=\]**

**A_6 - Keine automatische Kombination von `/leave` und `/forget`**

TI-M Fachdienste DÜRFEN bei Aufruf der API [`/leave`] NICHT automatisch ein
[`/forget`] ausführen. **\[\<=\]**

Haben alle Teilnehmer eines Homeservers einen privaten Raum verlassen und per
[`/forget`] clientseitig entfernt, so muss dieser Raum mit seinen Inhalten auch
serverseitig gelöscht werden. Dies folgt direkt aus dem DSGVO-Prinzip der
Datensparsamkeit und der Tatsache, dass Nutzer diese Räume nicht mehr betreten
können und auch auf ihren Geräten zur Löschung freigegeben haben.

**A_7 - Serverseitiges Löschen nach `/forget`**

TI-M Fachdienste MÜSSEN einen Raum und dessen Inhalte lokal löschen, wenn:

- der Raum privat ist und
- keiner der Nutzer des Homservers im Raum die Membership `invite` oder `join`
  hat und
- alle Nutzer des Homeservers, deren Membership im Raum `leave` oder `ban` ist,
  den Raum per [`/forget`] von ihren Clients entfernt haben.

Diese Löschung MUSS innerhalb von 7 Tagen ab letztem [`/forget`] erfolgen.
**\[\<=\]**

Die Regelungen zum serverseitigen Löschen bei [`/leave`] *ohne* [`/forget`]
richten sich nach dem vorigen Abschnitt.

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
Verlusts von eigentlich relevanten Nachrichten für andere Gesprächsteilnehmer.
Als Kompromiss werden Redactions daher zwar erlaubt. Sie müssen im Client aber
stets mit einem Warnhinweis versehen werden[^1].

**A_8 - Nachrichtenbasiertes Löschen per Redaction**

Clients MÜSSEN ihren Nutzern erlauben eigene Nachrichten per Redaction zu
löschen. Dabei MUSS der Nutzer vor jedem Auslösen einer Löschung per Warnhinweis
darauf hingewiesen werden, dass die Nachricht irreversibel und für alle
Gesprächsteilnehmer gelöscht wird.

Ist eine zu löschende Nachricht Ausgangspunkt einer Kette von [Event
Replacements], so MÜSSEN alle Events dieser Kette redacted werden. Dies kann
z.B. über mehrere einzelne Redactions oder einen Mechanismus wie in [MSC3912]
geschehen. **\[\<=\]**

**A_9 - Kennzeichnung gelöschter Nachrichten**

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

### DSGVO & Datenschutz

TI-M Clients und Fachdienste unterliegen der DSGVO. Da dies bereits gesetzlich
geregelt ist, bedarf es hierfür prinzipiell keiner neuen Anforderungen in der
TI-M Spezifikation. Gleichzeitig muss validiert werden, dass die zuvor
eingeführten neuen Anforderungen nicht im Widerspruch zur DSGVO oder daraus
abgeleiteten Aktionen stehen. Weiterhin kann eine konkrete Anwendung der DSGVO
auf TI-M und Matrix Hersteller und Anbieter bei einer rechtskonformen Umsetzung
unterstützen.

#### Personenbezogene Daten und Anwendungsbereiche der DSGVO

Bei der Verwendung von TI-M fallen vielfältige personenbezogene Daten im Sinne
von [DSGVO Art. 4 Nr. 1] an. Unter anderem sind, abgesehen von
Funktionsaccounts, alle TI-M Accounts identifizierbar, bei Versicherten
insbesondere durch Verwendung der KVNR in der Matrix-ID. Zu den
personenbezogenen Daten gehören daher *mindestens* alle Informationen, die sich
mit der Matrix-ID eines Nutzers verknüpfen lassen:

- Versendete Events und Medien (egal ob verschlüsselt oder nicht)
- Raummitgliedschaften ([`m.room.member`]) und andere gesendete State Events
- [Profile]
- [(Room) Account Data]
- [Devices]
- [Key Backups]
- [Veröffentlichte Schlüssel]
- ...

Die "Verarbeitung" dieser Daten umfasst nach [DSGVO Art. 4 Nr.
2][DSGVO Art. 4 Nr. 1] auch deren Speicherung auf TI-M Clients und Homeservern.
Hierbei sind natürliche Personen in ausschließlich persönlicher Tätigkeit nach
\[DSGVO Art. 2 Absatz 2c\] ausgenommen. Auf Versicherte als Verarbeiter von
Daten gestützt durch ihre TI-M ePA Clients und Homeserver findet die DSGVO daher
keine Anwendung. Alle anderen Komponenten der obigen Architekturskizze hingegen,
insbesondere auch die Mitarbeiter des Gesundheitswesens und deren Archivsysteme
unterliegen gegenüber Versicherten der DSGVO. Es ist dabei unerheblich, dass die
Archivsysteme selbst nicht Teil der TI sind.

#### Auskunftsrecht

Nach [DSGVO Art. 15] haben TI-M Nutzer gegenüber dem Anbieter ihres Homeservers
ein Auskunftsrecht. Dies umfasst sowohl die Herausgabe der gesammelten
personenbezogenen Daten selbst als auch die Auflistung von Dritten,
denengegenüber diese Daten offengelegt wurden.

Im TI-M Kontext werden diese Dritten insbesondere auch andere Homeserver sein.
Die Offenlegung von Daten gegenüber diesen Servern kann einerseits vom Nutzer
selbst verursacht worden sein, z. B. durch das Versenden von Nachrichten.
Andererseits können Homeserver bestimmte Informationen über die
[Server-Server-API] auch selbstständig abfragen. Hierzu zählen u. a.
[historische Events], [freigegebene Profile] und [Schlüssel]. Um das
Auskunftsrecht erfüllen zu können ist daher eine fortlaufende serverseitige
Protokollierung dieser Offenlegungen notwendig.

Weiterhin kaskadiert das Auskunftsrecht auf Dritte mit denen Daten geteilt
wurden. Ein Versicherter hat daher auch ein Auskunftsrecht gegenüber dem
Mitarbeiter des Gesundheitswesens bzw. dem Anbieter dessen Homservers.

#### Zweckgebundene Verarbeitung

Die Verarbeitung von Daten muss nach [DSGVO Art. 6] stets zweckgebunden sein. Im
Kontext von TI-M lassen sich innerhalb der TI zwei verschiedene Zwecke
identifizieren:

1.  Die medizinische Kommunikation selbst (nach [SGB 5 § 342 Absatz 2 Nr. 2])
2.  Die Ausleitung der Kommunikation in ein externes Archivsystem zur
    medizinischen Dokumentation (z. B. nach [BGB § 630f Absatz 3])

Für das außerhalb der TI befindliche Archivsystem gibt es nur einen
Verarbeitungszweck:

1.  Die Vorhaltung der Kommunikation zur medizinischen Dokumentation (z. B. nach
    [BGB § 630f Absatz 3])

Bevor Daten verarbeitet werden dürfen müssen TI-M Anbieter von ihren Nutzern
eine Einwilligung einholen und dabei die zugrundeliegenden Zwecke darlegen. Eine
gegebene Einwilligung können Nutzer allerdings jederzeit widerrufen.

Weiterhin können Verarbeitungszwecke im Laufe der Zeit gegenstandslos werden. So
entfällt der Zweck der Ausleitung von Daten in ein Archivsystem offensichtlich
sobald diese Ausleitung stattgefunden hat. Der Zweck der Vorhaltung von Daten in
einem Archivsystem wiederum entfällt sobald die zugrundeliegende gesetzliche
Vorhaltedauer abgelaufen ist. Der Zweck der medizinischen Kommunikation
schließlich entfällt wenn z. B. ein Behandlungsgespräch abgeschlossen ist.

#### Löschung und Recht auf Vergessenwerden

Eine Verarbeitung von Daten ohne zugrundeliegende Verarbeitungszwecke ist nach
[DSGVO Art. 6] nicht zulässig. Sind alle Zwecke entfallen, so müssen die
personenbezogenen Daten daher gelöscht werden. Dabei ist gemäß [BDSG § 58 Absatz
3] anstatt einer Löschung auch eine Einschränkung der Verarbeitung erlaubt.

Des Weiteren haben TI-M Nutzer nach [DSGVO Art. 17 Absatz 1][DSGVO Art. 17] das
Recht die Löschung ihrer personenbezogenen Daten zu verlangen ("Recht auf
Vergessenwerden"). Die Löschung muss daraufhin vollzogen werden, es sei denn es
existieren vom Widerruf der Nutzereinwilligung unberührte Verarbeitungszwecke
wie z. B. gesetzliche Vorhaltefristen.

Änlich zum Auskunftsrecht kaskadiert auch das Recht auf Vergessenwerden auf
Dritte mit denen personenbezogene Daten geteilt wurden. Gemäß [DSGVO Art. 17
Absatz 2][DSGVO Art. 17] müssen entsprechende Anfragen zudem auch an diese
Dritten weitergeleitet werden. Dies kann wahlweise technisch oder
organisatorisch erfolgen.

Zur Illustration der Löschung betrachten wir im Folgenden verschiedene
Szenarien.

##### Szenario 1: Versicherter fordert Recht auf Vergessenwerden vom Anbieter seines TI-M ePA Homeservers ein

Für den TI-M ePA Anbieter ist der einzige Verarbeitungszweck die medizinische
Kommunikation. Mit der Anfrage des Versicherten entfällt dieser Zweck, so dass
die personenbezogenen Daten des Versicherten auf dem Homeserver gelöscht werden
müssen.

Der Anbieter kann die meisten der oben aufgelisteten Daten direkt und ohne
schädliche Einflüsse auf andere Nutzer oder Server löschen.

Events stellen einen Sonderfall dar denn sie bilden eine serverseitige
Datenstruktur, die von allen Raumteilnehmern desselben Homeservers geteilt und
zudem über die Föderation verbreitet wird. Der TI-M ePA Anbieter hat dabei keine
Einsicht darin ob die Verarbeitungszwecke auf den empfangenden TI-M Pro
Homeservern durch den Widerruf der Einwilligung entfallen oder nicht. Unterliegt
die Kommunikation z. B. Regelungen der medizinischen Dokumentation und wurde sie
noch nicht archiviert, so dürfen die Events dort bis zur Archivierung weiterhin
verarbeitet werden.

Hieraus ergibt sich für den TI-M ePA Anbieter, dass eine Löschung aller vom
Versicherten versendeten Events per Redactions nicht zulässig ist. Stattdessen
muss der Versicherte aus allen seinen Räumen entfernt werden, analog zu einem
[`/leave`] mit anschließendem [`/forget`]. Nach A_7 müssen die Räume samt der
enthaltenen Events dann lokal gelöscht werden, sofern sich kein weiterer
Versicherter desselben Homeservers darin befindet (z. B. wegen einer
Vertreterregelung).

Weiterhin ist zu beachten, dass gängige Matrix Homeserver Nutzer-Accounts selbst
in der Regel nicht löschen sondern nur deaktivieren. In der öffentlichen
Föderation wäre eine Löschung im allgemeinen unerwünscht, da sich dann jemand
anderes unter derselben Matrix-ID registrieren und den vorigen Besitzer
personifizieren könnte. Für TI-M ePA Homeserver besteht dieses Risiko allerdings
nicht weil die Matrix-ID nach [A_25706] aus der KVNR des Versicherten abgeleitet
wird. Der TI-M ePA Anbieter kann und muss daher auch den Account selbst löschen.
Aktuell gibt es hierfür in Matrix keine dedizierte API. Hersteller können aber
eigene APIs implementieren oder das Verhalten von
[`/_matrix/client/v3/account/deactivate`] über Module anpassen sofern ihre
Homeserverimplementierung dies zulässt. In einem zukünftigen Proposal sollte u.
U. eine einheitliche API eingeführt werden.

Abschließend muss der TI-M ePA Anbieter die Anfrage des Versicherten an alle
Homeserver weiterleiten mit denen er Daten des Versicherten geteilt hat. Auch
hierfür gibt es in Matrix momentan keinen technischen Mechanismus. Die
Kommunikation kann daher nur manuell über die in [`/.well-known/matrix/support`]
hinterlegten Wege erfolgen. Mit einem zukünftigen Proposal sollte u. U. auch
hier eine dedizierte API geschaffen werden.

##### Szenario 2: Versicherter fordert Recht auf Vergessenwerden von seinem Arzt ein

Fällt die Kommunikation unter Regelungen zur medizinischen Dokumentation, so
muss der Arzt sie zunächst in sein Archivsystem übertragen. Anschließend sind
für ihn innerhalb der TI alle Verarbeitungszwecke entfallen und er muss die
personenbezogenen Daten über seinen TI-M Pro Client löschen. Redactions als
Mittel zur Löschung verbieten sich auch hier denn der Arzt kann nicht wissen ob
der Versicherte auch eine Anfrage gegen den Anbieter seines eigenen Homeservers
gestellt hat. Stattdessen muss der Arzt die Räume des Versicherten per
[`/leave`] und [`/forget`] verlassen. Nach A_7 führt dies dann auch zur Löschung
der Räume und der darin enthaltenen Events auf dem TI-M Pro Homeserver des
Arztes.

Die im Archivsystem befindlichen personenbezogen Daten des Versicherten hingegen
darf der Arzt nicht löschen solange die zugrundeliegende gesetzliche
Vorhaltedauer nicht verstrichen ist.

##### Szenario 3: Mitarbeiter einer Krankenkasse sieht Unterhaltung mit einem Versicherten als abgeschlossen an

Beim Zweck der medizinischen Kommunikation ist es wichtig zu beachten, dass
deren Abschluss von den Teilnehmern unterschiedlich beurteilt werden kann.
Aufgrund der dezentralen Architektur von TI-M lässt sich dies jedoch gut
handhaben.

Betrachtet der Mitarbeiter das Gespräch als beendet entfällt für ihn der einzige
Verarbeitungszweck. Der Versicherte kann gleichzeitig eine andere Meinung über
den Abschluss der Kommunikation haben. Analog zu oben sind für den Mitarbeiter
Redactions daher als Instrument zur Löschung ausgeschlossen. Stattdessen muss
der Mitarbeiter den Raum auf seinem TI-M Pro Client per [`/leave`] und
[`/forget`] verlassen was wegen A_7 wieder zu einer Löschung des Raums auf
seinem TI-M Pro Homeserver führt.

Auf dem TI-M ePA Homeserver des Versicherten kann der Raum dadurch ungehindert
weiterbestehen.

#### Erasure Requests

Die Matrix Foundation hat zur Behandlung von datenschutzrechtlichen
Löschanfragen sogenannte [Erasure requests] definiert. Dieser Mechanismus
orientiert sich an der E-Mail-Architektur und ist speziell für die öffentliche
Föderation entworfen worden. Da in diesem Verfahren zu keinem Zeitpunkt eine
Löschung oder Einschränkung der Verarbeitung für vormalige Empfänger von Events
stattfindet, ist es zur Behandlung von Anfragen zum Recht auf Vergessenwerden im
TI-M-Kontext ungeeignet.

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
  [`/leave`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3roomsroomidleave
  [`/sync`]: https://spec.matrix.org/v1.13/client-server-api/#get_matrixclientv3sync
  [Filter]: https://spec.matrix.org/v1.13/client-server-api/#filtering
  [`/forget`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3roomsroomidforget
  [`forget_rooms_on_leave`]: https://element-hq.github.io/synapse/latest/usage/configuration/config_documentation.html#forget_rooms_on_leave
  [Redactions]: https://spec.matrix.org/v1.13/client-server-api/#redactions
  [Event Replacements]: https://spec.matrix.org/v1.13/client-server-api/#event-replacements
  [MSC3912]: https://github.com/matrix-org/matrix-spec-proposals/pull/3912
  [Reporting Content]: https://spec.matrix.org/v1.13/client-server-api/#reporting-content
  [DSGVO Art. 4 Nr. 1]: https://dsgvo-gesetz.de/art-4-dsgvo/
  [`m.room.member`]: https://spec.matrix.org/v1.13/client-server-api/#mroommember
  [Profile]: https://spec.matrix.org/v1.13/client-server-api/#profiles
  [(Room) Account Data]: https://spec.matrix.org/v1.13/client-server-api/#client-config
  [Devices]: https://spec.matrix.org/v1.13/client-server-api/#get_matrixclientv3devices
  [Key Backups]: https://spec.matrix.org/v1.13/client-server-api/#server-side-key-backups
  [Veröffentlichte Schlüssel]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3keysupload
  [DSGVO Art. 15]: https://dsgvo-gesetz.de/art-15-dsgvo/
  [Server-Server-API]: https://spec.matrix.org/v1.13/server-server-api/
  [historische Events]: https://spec.matrix.org/v1.13/server-server-api/#backfilling-and-retrieving-missing-events
  [freigegebene Profile]: https://spec.matrix.org/v1.13/server-server-api/#get_matrixfederationv1queryprofile
  [Schlüssel]: https://spec.matrix.org/v1.13/server-server-api/#post_matrixfederationv1userkeysquery
  [DSGVO Art. 6]: https://dsgvo-gesetz.de/art-6-dsgvo/
  [SGB 5 § 342 Absatz 2 Nr. 2]: https://www.gesetze-im-internet.de/sgb_5/__342.html
  [BGB § 630f Absatz 3]: https://www.gesetze-im-internet.de/bgb/__630f.html
  [BDSG § 58 Absatz 3]: https://dsgvo-gesetz.de/bdsg/58-bdsg/
  [A_25706]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_ePA/latest/#A_25706
  [`/_matrix/client/v3/account/deactivate`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3accountdeactivate
  [`/.well-known/matrix/support`]: https://spec.matrix.org/v1.13/client-server-api/#getwell-knownmatrixsupport
  [Erasure requests]: https://matrix.org/blog/2024/06/regulatory-update/
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
