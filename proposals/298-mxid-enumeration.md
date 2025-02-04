# GSC298: Verhinderung der Enumeration von MXIDs

Durch [`/_matrix/client/v3/keys/query`] lassen sich MXIDs an der Client-Server
API enumerieren. Für TI-M ePA Nutzer enthalten MXIDs Krankenversichertennummer,
die sich dadurch mit anderen Metadaten wie z. B. den ebenfalls am Endpunkt
ausgelieferten Gerätenamen korrelieren lassen.

Das vorliegende Proposal unterbindet diese Metadaten in Fällen, in denen sie
nicht zwingend notwendig sind.

## Änderungsvorschlag

Aufbauend auf [MSC4263], dürfen Fachdienste Schlüssel anderer Nutzer nur noch
ausliefern wenn der anfragende Nutzer die MXID des angefragten Nutzers auch über
`m.room.membership` Events in Räumen sehen kann.

**A_1 - Key Queries nur mit gemeinsamen oder öffentlichen Räumen**

TI-M Fachdienste MÜSSEN in ihrer Antwort am Endpunkt
[`/_matrix/client/v3/keys/query`] MXIDs von Nutzern unterdrücken, für die kein
`m.room.membership` Event in Räumen des anfragenden Nutzers oder dem Fachdienst
bekannten öffentlichen Räumen existiert. Der konkrete `membership` Wert, also z.
B. `join` oder `leave`, ist dabei unerheblich. **\[\<=\]**

## Sicherheit und Datenschutz

Durch das Blockieren der Schlüsselabfragen in nicht-notwendigen Fällen werden
unnötige Metadaten vermieden und der Datenschutz gestärkt.

Am Endpunkt [`/_matrix/federation/v1/user/keys/query`] in der Server-Server API
ist ein Enumerieren von MXIDs weiterhin möglich. Diese API steht aber nur
Servern innerhalb der TI-Föderation zur Verfügung wodurch der Angriffsvektor
deutlich verkleinert wird.

## Kompatibilität & Migration

Clients haben ohne gemeinsame Räume keinen Grund die Schlüssel anderer Nutzer
abzufragen. Daher sollte es durch die beschriebene Einschränkung keine
Kompatibilitätsprobleme geben.

## Alternativen

Keine.

  [`/_matrix/client/v3/keys/query`]: https://spec.matrix.org/v1.13/client-server-api/#post_matrixclientv3keysquery
  [MSC4263]: https://github.com/matrix-org/matrix-spec-proposals/pull/4263
  [`/_matrix/federation/v1/user/keys/query`]: https://spec.matrix.org/v1.13/server-server-api/#post_matrixfederationv1userkeysquery
