# GSC302: Statische Strings in Annotations bzw. Reactions

Das `key` Feld in `m.reaction` Events ist nach [Matrix-Spezifikation] ein
nicht-längenbeschränkter String. Da dieses Feld innerhalb einer `m.annotation`
Relation auftritt, ist es immer unverschlüsselt. Hieraus ergibt sich ein
Sicherheitsproblem da Nutzer unbeabsichtigt sensible Informationen über dieses
Feld teilen könnten. [A_26227] und [A_26228] behandeln dieses Problem indem sie
den Inhalt von `key` auf ein einzelnes Emoji bzw. Unicode-Zeichen beschränken.

> **A_26227-01 - Längenbegrenzung von Annotationen in Reactions**
>
> Der TI-M Client MUSS sicherstellen, dass der Wert des Attributes key im
> Event-Content in Events des Typs m.reaction beim Erzeugen die Länge eines
> einzelnen Unicode-Characters bzw. dessen Code Point Repräsentation nicht
> überschreitet. **\[\<=\]**

> **A_26228-01 - Längenbegrenzung von Annotationen in Reactions**
>
> Der TI-M FD MUSS sicherstellen, dass der Wert des Attributes key im
> Event-Content in Events des Typs m.reaction nur ein Emoji darstellt, sonst
> MUSS der Request mit dem Response Code 400 und dem Error Code M_BAD_JSON
> abgewiesen werden. **\[\<=\]**

Diese Einschränkung ist zwar zielführend aber gleichzeitig zu streng, denn sie
unterbindet die sinnvolle Nutzung von Reactions mit nicht-sensitiven Inhalten
die länger als ein Emoji sind. Ein Beispiel für solche nicht-sensitiven Inhalte
sind statische Strings, z. B. zur Markierung von Bildern als "Heruntergeladen".

![Heruntergeladen Reaction]

Das vorliegende Proposal schwächt die bestehenden Anforderungen ab um die
Verwendung statischer Strings in Reactions zu ermöglichen.

## Änderungsvorschlag

[A_26227] wird aufgeweicht um das Senden statischer Strings zu erlauben.

> **A_26227-02 - Beschränkung von Annotationen in Reactions**
>
> Der TI-M Client MUSS sicherstellen, dass der Wert des Attributes `key` im
> Event-Content von `m.reaction` Events nur vordefinierte, statische Strings
> ohne Personenbezug enthält. **\[\<=\]**

[A_26228] wird nicht angepasst sondern gestrichen da der Server alleine nicht
entscheiden kann ob ein String statisch ist oder nicht.

## Sicherheit und Datenschutz

Für Akteure, die konforme TI-M Clients verwenden ist die Prüfung im Client
hinreichend. Durch den Wegfall der Prüfung am Server, könnte ein Akteur zwar
Reactions mit sensitiven Inhlaten versenden, indem er die API außerhalb eines
Clients bedient. So ein Akteur wäre von dem Szenario des *unbeabsichtigten*
Versendens von sensiblen Inhalten aber gar nicht betroffen.

## Kompatibilität & Migration

Wegen [A_26228] werden aktuelle Fachdienste Reactions mit statischen Strings
länger als 1 Zeichen ablehnen. Ein Client, der dieses Proposal implementiert
kann zwar vorab nicht erkennen welcher Version der Spezifikation der Server
unterliegt. Er kann allerdings auf den zurückgelieferten Fehlercode entsprechend
reagieren.

Clients könnten für bestimmte statische Reactions ein spezielles UI
implementieren oder darauf mit einer speziellen Aktion reagieren. Da dieses
Proposal keine konkreten statischen Strings standardisiert wäre dieses Verhalten
vorerst auf Clients desselben Herstellers beschränkt. Dies erscheint für den
Anfang akzeptabel denn konkrete einheitliche Werte können spezifiziert werden
sobald deren Relevanz über Hersteller hinweg bekannt ist. Ohne das Senden von
statischen Strings generell zu erlauben kann diese Relevanz aber nur schwer
bestimmt werden.

## Alternativen

Keine.

  [Matrix-Spezifikation]: https://spec.matrix.org/v1.13/client-server-api/#event-annotations-and-reactions
  [A_26227]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_26227-01
  [A_26228]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_26228-01
  [Heruntergeladen Reaction]: 302-statische-strings-in-reactions.png
    "Heruntergeladen Reaction"
