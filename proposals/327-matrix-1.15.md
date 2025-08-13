# GSC327: TI-M on Matrix 1.15

TI-M basiert im aktuellen Release auf [Matrix 1.11], welches im Juni 2024 veröffentlicht wurde.
Seitdem gab es vier weitere Matrix-Release. Um neue Features und Verbesserungen in TI-M nutzbar zu
machen ist ein Update der Matrix Version in TI-M erforderlich. Dieses Proposal listet die für TI-M
relevanten Matrix-Neuerungen auf und leitet, sofern notwendig, entsprechende Änderungen an der
Spezifikation ab.

## Änderungsvorschlag

### `server_name` → `via` Migration

Der `server_name` Query-Parameter auf [`/_matrix/client/v3/join/{roomIdOrAlias}`] and
[`/_matrix/client/v3/knock/{roomIdOrAlias}`] wurde in [Matrix 1.12] als deprecated markiert und
durch einen neuen Parameter `via` ersetzt. Server müssen dabei weiterhin beide Parameter akzeptieren
aber `via` bevorzugen.

> - Deprecate the `server_name` query parameter on `POST /_matrix/client/v3/join/{roomIdOrAlias}`
>   and `POST /_matrix/client/v3/knock/{roomIdOrAlias}`, as per [MSC4156]. ([\#1933])
> - Add `via` query parameter on `POST /_matrix/client/v3/join/{roomIdOrAlias}` and
>   `POST /_matrix/client/v3/knock/{roomIdOrAlias}`, as per [MSC4156]. ([\#1933])

In [Matrix 1.14][Matrix 1.12] wurde der `server_name` Parameter dann gänzlich aus der Spec entfernt.

> Remove `server_name` parameter from `/_matrix/client/v3/join/{roomIdOrAlias}` and
> `/_matrix/client/v3/knock/{roomIdOrAlias}`, as per [MSC4213]. ([\#2059])

Da es bereits TI-M Clients im Feld gibt und diese `via` nicht kennen, müssen TI-M Fachdienste
`server_name` weiterhin als Fallback unterstützen.

> **A_1 – Weitere Unterstützung von `server_name`**
>
> TI-M Fachdienste MÜSSEN bei den Requests `POST /_matrix/client/v3/join/{roomIdOrAlias}` und
> `POST /_matrix/client/v3/knock/{roomIdOrAlias}` den Parameter `server_name` so wie in Matrix v1.12
> weiterhin unterstützen. **\[\<=\]**
>
> Hinweis: `server_name` wurde in Matrix v1.14 entfernt und durch den in Matrix v1.12 eingeführten
> `via` Parameter ersetzt. Die Unterstützung von `server_name` nach Matrix v1.12 beinhaltet, dass
> `via` serverseitig bevorzugt wird falls beide Parameter gesetzt sind.

Clients können über [`/_matrix/client/versions`] erkennen ob ihr Server `via` unterstützt und
wahlweise `via` oder `server_name` verwenden. Alternativ können Clients auch einfach beide Parameter
gleichzeitig und mit identischem Inhalt setzen. TI-M-seitig bedarf es hier keiner weiteren Regelung.

### Einschränkung von Profilabfragen

In [Matrix 1.12] wurde für Server die Möglichkeit eingeführt Profilabfragen einzuschränken. Abfragen
müssen nur noch beantwortet werden wenn der anfragende und der angefragte Nutzer gemeinsame Räume
haben oder wenn der angefragte Nutzer Mitglied eines öffentlichen Raumes ist. In allen anderen
Fällen dürfen Server Anfragen mit 403 `M_FORBIDDEN` ablehen.

> - Add 403 responses on `GET /_matrix/client/v3/profile/{userId}/avatar_url` and
>   `GET /_matrix/client/v3/profile/{userId}/displayname`, as per [MSC4170]. ([\#1867])
> - Add 403 response on `GET /_matrix/federation/v1/query/profile`, as per [MSC4170]. ([\#1867])

Die Neuregelung der Fälle in denen Profilabfragen möglich sein müssen entspricht [A_26374] in TI-M
Basis. Diese Anforderung kann daher entfallen.

Weiterhin verbietet [A_26290] auf ePA-Fachdiensten Profilabfragen in allen anderen Fällen. Diese
Anforderung ist bis auf den fehlenden Fehlercode konform mit der neuen Matrix-Spezifikation. Der
Fehlercode wird daher in die Anforderung eingefügt.

> **A_26290-1 - Verbot von Profilabfragen ohne gemeinsame Räume**
>
> Der TI-M Fachdienst ePA MUSS Requests zu den folgenden Endpunkten mit einer HTTP 403 Response *und
> dem Fehlercode `M_FORBIDDEN`* ablehnen, sofern der anfragende Nutzer keine gemeinsamen Räume mit
> dem angefragten Nutzer hat:
>
> - `GET /_matrix/client/v3/profile/{userId}`
> - `GET /_matrix/client/v3/profile/{userId}/avatar_url`
> - `GET /_matrix/client/v3/profile/{userId}/displayname`
>
> **\[\<=\]**

### `dont_notify` und `coalesce` Push Rule Actions

In [Matrix 1.12] wurde klargestellt, dass die Push Rule Actions `dont_notify` und `coalesce` nicht
abzulehnen sondern zu ignorieren sind.

> - Clarify that the deprecated `dont_notify` and `coalesce` push rule actions MUST be ignored, not
>   rejected. ([\#1890])

Da gegenteilige Implementierungen nicht bekannt sind gibt es für TI-M an dieser Stelle eigentlich
nichts zu tun. Gleichzeitig fällt aber auf, dass mit [A_26193] die Verwendung dieser Actions für
TI-M Clients explizit verboten wurde. Dies erscheint überflüssig, da diese Actions von jedem Server
ignoriert werden und ohnehin nicht mehr Teil der Matrix-Spezifikation sind. [A_26193] wird daher
entfernt.

### OAuth 2.0 APIs

In [Matrix 1.15] wurden neue OAuth APIs eingeführt.

> Add the OAuth 2.0 based authentication API, as per [MSC3861] and its sub-proposals. ([\#2141],
> [\#2148], [\#2149], [\#2150], [\#2151], [\#2159], [\#2164])

Im Vergleich zu den von TI-M verwendeten Authentication APIs bieten die OAuth APIs den Vorteil, dass
PKCE und State verpflichtend integriert sind. Des Weiteren erleichtert die Verwendung von OAuth die
Nachnutzung bestehender Libraries. Nachteilig ist hingegen, dass die neuen APIs nicht mit der
[User-Interactive Authentication] kompatibel sind. Konkret davon betroffen ist allerdings nur das
Reset der Cross-Signing Keys, wofür es mit [MSC4312] einen Workaround gibt.

In Summe überwiegen die Vorteile der neuen APIs. Neue TI-M Clients werden daher verpflichtet die
OAuth APIs zu verwenden.

> **A_2 – Verwendung der OAuth-APIs**
>
> TI-M Clients MÜSSEN sofern ihr Fachdienste Matrix 1.15 unterstützt die [OAuth APIs] zur
> Authentifizierung nutzen. **\[\<=\]**

Da wir bereits Clients im Feld haben, die Matrix 1.15 nicht unterstützen müssen Fachdienste die
Authentifizierung allerdings weiterhin auch über die alten APIs erlauben.

> **A_3 – Authentication APIs**
>
> TI-M Fachdienste MÜSSEN die Authentifizierung sowohl über die [OAuth APIs] als auch über die
> [Legacy APIs] erlauben. **\[\<=\]**

Weiterhin werden Clients und Fachdienste verpflichtet zum Reset der Cross-Signing Keys den
Workaround aus [MSC4312] umzusetzen.

> **A_4 – Cross-Signing-Reset & OAuth APIs**
>
> TI-M Clients und Fachdienste MÜSSEN zum Reset der Cross-Signing-Keys das Verfahren aus [MSC4312]
> implementieren sofern die Authentifizierung über die [OAuth APIs] erfolgt ist. **\[\<=\]**

## Nennenswerte Änderungen ohne Anpassungen in TI-M

### Raumversion 11

In [Matrix 1.14] wurde Raum Version 11 als neuer Standard festgelegt

> Update the default room version to 11, as per [MSC4239]. ([\#2105])

Für TI-M gelten wegen [Kapitel 5.4 in TI-M Basis] folgende Regeln:

- Version 9, 10 und 11 müssen unterstützt werden
- Version 10 muss als Default verwendet werden
- Räume mit Version 11 dürfen nicht erzeugt werden

Leider haben wir weiterhin Implementierungen aus der alten TI-M 1.x Linie im Feld. Diese basieren
auf Matrix 1.3 und kennen Raumversion 11 nicht. Hier gibt es also vorerst keine Änderung.

### Mark as unread

Räume können seit [Matrix 1.12] explizit als ungelesen markiert werden.

> - Add support for marking rooms as unread, as per [MSC2867]. ([\#1895], [\#1941])

Dieses Feature ist Teil des Moduls [Receipts] und kann in TI-M ohne Einschränkung verwendet werden.

## Sicherheit und Datenschutz

Die Verwendung der neuen [OAuth APIs] erhöht durch PKCE die Sicherheit des Login-Verfahrens.

## Kompatibilität & Migration

Alle obigen Änderungen garantieren Kompatibilität zu bestehenden Implementierungen.

## Alternativen

Keine.

  [Matrix 1.11]: https://spec.matrix.org/v1.15/changelog/v1.11/
  [`/_matrix/client/v3/join/{roomIdOrAlias}`]: https://spec.matrix.org/v1.15/client-server-api/#post_matrixclientv3joinroomidoralias
  [`/_matrix/client/v3/knock/{roomIdOrAlias}`]: https://spec.matrix.org/v1.15/client-server-api/#post_matrixclientv3knockroomidoralias
  [Matrix 1.12]: https://spec.matrix.org/v1.15/changelog/v1.12/
  [MSC4156]: https://github.com/matrix-org/matrix-spec-proposals/pull/4156
  [\#1933]: https://github.com/matrix-org/matrix-spec/issues/1933
  [MSC4213]: https://github.com/matrix-org/matrix-spec-proposals/pull/4213
  [\#2059]: https://github.com/matrix-org/matrix-spec/issues/2059
  [`/_matrix/client/versions`]: https://spec.matrix.org/v1.15/client-server-api/#get_matrixclientversions
  [MSC4170]: https://github.com/matrix-org/matrix-spec-proposals/pull/4170
  [\#1867]: https://github.com/matrix-org/matrix-spec/issues/1867
  [A_26374]: https://gemspec.gematik.de/prereleases/Draft_TI-Messenger_24_3/gemSpec_TI-M_Basis_V1.1.0_CC/#A_26374
  [A_26290]: https://gemspec.gematik.de/prereleases/Draft_TI-Messenger_24_3/gemSpec_TI-M_ePA_V1.1.0_CC/#A_26290
  [\#1890]: https://github.com/matrix-org/matrix-spec/issues/1890
  [A_26193]: https://gemspec.gematik.de/prereleases/Draft_TI-Messenger_24_3/gemSpec_TI-M_Basis_V1.1.0_CC/#A_26193
  [Matrix 1.15]: https://spec.matrix.org/v1.15/changelog/v1.15/
  [MSC3861]: https://github.com/matrix-org/matrix-spec-proposals/pull/3861
  [\#2141]: https://github.com/matrix-org/matrix-spec/issues/2141
  [\#2148]: https://github.com/matrix-org/matrix-spec/issues/2148
  [\#2149]: https://github.com/matrix-org/matrix-spec/issues/2149
  [\#2150]: https://github.com/matrix-org/matrix-spec/issues/2150
  [\#2151]: https://github.com/matrix-org/matrix-spec/issues/2151
  [\#2159]: https://github.com/matrix-org/matrix-spec/issues/2159
  [\#2164]: https://github.com/matrix-org/matrix-spec/issues/2164
  [User-Interactive Authentication]: https://spec.matrix.org/v1.15/client-server-api/#user-interactive-authentication-api
  [MSC4312]: https://github.com/matrix-org/matrix-spec-proposals/pull/4312
  [OAuth APIs]: https://spec.matrix.org/v1.15/client-server-api/#oauth-20-api
  [Legacy APIs]: https://spec.matrix.org/v1.15/client-server-api/#legacy-api
  [Matrix 1.14]: https://spec.matrix.org/v1.15/changelog/v1.14/
  [MSC4239]: https://github.com/matrix-org/matrix-spec-proposals/pull/4239
  [\#2105]: https://github.com/matrix-org/matrix-spec/issues/2105
  [Kapitel 5.4 in TI-M Basis]: https://gemspec.gematik.de/docs/gemSpec/gemSpec_TI-M_Basis/latest/#5.4
  [MSC2867]: https://github.com/matrix-org/matrix-spec-proposals/pull/2867
  [\#1895]: https://github.com/matrix-org/matrix-spec/issues/1895
  [\#1941]: https://github.com/matrix-org/matrix-spec/issues/1941
  [Receipts]: https://spec.matrix.org/v1.15/client-server-api/#receipts
