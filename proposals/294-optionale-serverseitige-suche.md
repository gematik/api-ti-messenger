# Optionale serverseitige Suche

Das Matrix-Modul [Server Side Search] ist nach [A_25395] für alle TI-M Clients
verpflichtend. Mit dem Endpunkt [`/_matrix/client/v3/search`] aus diesem Modul
können Clients eine Volltextsuche auf Events in allen Räumen eines Benutzers
ausführen. Diese Operation auf dem Server auszuführen hat den Vorteil, dass der
Client für die Suche nicht erst die komplette Historie aller Räume laden muss.
Aufgrund der Ende-zu-Ende-Verschlüsselung kann der Server allerdings nur
unverschlüsselte Events durchsuchen, in verschlüsselten Räumen also nur State
Events. Durch die zur Verfügung stehenden [Kategorien], sind das konkret nur die
State Events für Raumname und Topic.

Bei verschlüsselten Räumen erscheint eine auf Name und Topic beschränkte
serverseitige Suche nur bedingt hilfreich. Das liegt u. a. auch daran, dass die
Raumnamen üblicherweise für die Raumliste im Client ohnehin schon zur Verfügung
stehen.

In unverschlüsselten Räumen kann die serverseitige Suche ein nützliches Feature
sein. Wegen [A_25324] aus TI-M Basis, [A_26015] und [A_25996] aus TI-M ePA sowie
[A_25325] aus TI-M Pro dürfen unverschlüsselte Räume allerdings nur durch TI-M
Pro Nutzer erzeugt werden. Weiterhin dürfen diese Räume wegen [A_26515] aus TI-M
Pro nur durch Nutzer desselben Homeservers betreten werden.

In Summe bedeutet das TI-M ePA Nutzer können keine unverschlüsselten Räume
haben. Eine verpflichtende Umsetzung des Moduls [Server Side Search] erscheint
für TI-M ePA Clients daher nicht sinnvoll.

Für TI-M Pro Nutzer können zwar unverschlüsselte Räume existieren. Diese Räume
werden allerdings nur einen Bruchteil der Kommunikation dieser Nutzer ausmachen.
Eine Verpflichtung zur Umsetzung der serverseitigen Suche erscheint daher auch
hier unnötig.

Im diesem Proposal wird die Verpflichtung zur Umsetzung dieses Moduls daher
abgeschwächt.

## Änderungsvorschlag

Das Modul [Server Side Search] wird in [A_25395] auf KANN herabgestuft. Clients
haben damit die Wahlmöglichkeit das Modul zu verwenden oder nicht.

## Sicherheit und Datenschutz

Keine Auswirkungen.

## Kompatibilität & Migration

Durch [A_26210] müssen Homeserver das Modul weiterhin umsetzen.
Kompatibilitätsprobleme kann es daher nicht geben.

## Alternativen

Das Modul könnte für TI-M Pro Clients als MUSS beibehalten werden. Die
Modultabelle aus [A_25395] in TI-M Basis müsste dafür durch eine neue
Anforderung in TI-M Pro überschrieben werden. Die daraus resultierende höhere
Komplexität der Spezifikation erscheint nicht gerechtfertigt da der Use Case für
dieses Feature wie oben beschrieben verhältnismäßig klein ist. Zudem ist die
Verwendung des Moduls in TI-M Pro durch die KANN-Klassifizierung nicht verboten.

  [Server Side Search]: https://spec.matrix.org/v1.13/client-server-api/#server-side-search
  [A_25395]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25395-01
  [`/_matrix/client/v3/search`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3search
  [Kategorien]: https://spec.matrix.org/v1.13/client-server-api/#search-categories
  [A_25324]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25324
  [A_26015]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_ePA/gemSpec_TI-M_ePA_V1.1.1/#A_26015
  [A_25996]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_ePA/gemSpec_TI-M_ePA_V1.1.1/#A_25996
  [A_25325]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Pro/gemSpec_TI-M_Pro_V1.0.1/#A_25325
  [A_26515]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Pro/gemSpec_TI-M_Pro_V1.0.1/#A_26515
  [A_26210]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_26210
