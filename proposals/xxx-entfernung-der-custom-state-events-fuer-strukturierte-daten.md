# Entfernung der Custom State Events für strukturierte Daten

TI-M Basis beschreibt in [Kapitel 5.5] die "TI-M spezifische Kommunikation".
Hierbei werden eigene State Events definiert, zu denen auch die Events
`de.gematik.tim.room.casereference.v1` und `de.gematik.tim.room.default.v1`
gehören. In diesen State Events sollen strukturierte Daten, wie z. B.
FHIR-Objekten zu einem Behandlungsfall gespeichert werden um damit spezielle
Anwendungsfälle zu unterstützen.

Vorraussetzung hierfür ist allerdings State Events verschlüsseln zu können was
zum jetzigen Zeitpunkt noch nicht im Protokoll vorgesehen ist[^1]. Aus diesem
Grund ist das [Unterkapitel 5.5.2] vollständig von der Umsetzung
ausgenommen[^2]. Die Tatsache, dass dieses Kapitel trotzdem in der Spezifikation
auftaucht und konkrete Anforderungen enthält führt dabei vielfach zu
Irritationen und Nachfragen.

Weiterhin müssen wegen [A_25756] und [A_25813] die State Events
`de.gematik.tim.room.casereference.v1` und `de.gematik.tim.room.default.v1` beim
Anlegen von Räumen gesendet werden, was auch im Rahmen der automatisierten
Zulassungstests geprüft wird. Auch dies verursacht wiederkehrend Verwirrung,
denn das zwangsweise Senden von Events, die gar nicht benutzt werden können
erscheint vollständig zwecklos.

Dieses Proposal schlägt eine inkrementelle Verbesserung dieser Situation durch
Entfernung bestimmter nutzloser Teile der Spezifikation an.

## Änderungsvorschlag

Das [Unterkapitel 5.5.2] wird vollständig aus der Spezifikation entfernt. Damit
verschwinden [A_25756] und das State Event
`de.gematik.tim.room.casereference.v1`.

Da dieses Unterkapitel bereits heute unverbindlich ist, ergeben sich dadurch
keine Auswirkungen auf die Zulassung oder bestehende Implementierungen. Der
einzige Mehrwert dieses Abschnittes ist, dass ein Ausblick auf zukünftige
Entwicklungen gegeben wird. Das ist aber zum einen nur bedingt Aufgabe der
Spezifikation und zum anderen funktioniert es an dieser Stelle nicht richtig.
Eine tatsächliche Verschlüsselung von State Events würde nämlich weitere
inkompatible Änderungen am Schema erfordern, z. B. durch die Verschlüsselung des
Event Typs. Durch die Entfernung dieses Unterkapitel wird die Spezifikation
vereinfacht und zukünftigen Missverständnissen vorgebeugt. Der zugehörige Text
kann unabhängig hiervon im Änderungsmanagementsystem der gematik verbleiben und
in Zukunft wiederverwendet werden.

Des Weiteren wird das State Event `de.gematik.tim.room.default.v1` aus den
Pflichtparametern für [`/createRoom`] in [A_25813] entfernt. Das verpflichtende
Senden eines nicht benutzbaren Events erscheint sinnlos. Eine zukünftige
Verwendung des Events würde auch hier inkompatible Änderungen erfordern. Das
heutige Senden des Events erfüllt daher auch keinen vorbereitenden Zweck. Da das
Event nicht verwendet werden kann, bewirkt eine Entfernung aus [A_25813] zudem
keine Inkompatibilitäten.

Abschließend wird [A_25816] mit derselben Argumentation gestrichen.

Das [Kapitel 5.5] ist mit diesen Änderungen zwar nicht abschließend
bereinigt[^3] aber zumindest deutlich klarer und weniger anfällig für Nachfragen
als in seiner heutigen Form.

[^1]: Eine mögliche Variante hierfür ist [MSC3414].

[^2]: Siehe hierzu den Hinweis ganz am Ende der Einleitung von [Kapitel 5.5].

[^3]: Siehe auch <https://github.com/gematik/api-ti-messenger/issues/272>.

  [Kapitel 5.5]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#5.5
  [Unterkapitel 5.5.2]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#5.5.2
  [A_25756]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25756-01
  [A_25813]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25813-01
  [`/createRoom`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3createroom
  [A_25816]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/gemSpec_TI-M_Basis_V1.1.1/#A_25816
  [MSC3414]: https://github.com/matrix-org/matrix-spec-proposals/pull/3414
