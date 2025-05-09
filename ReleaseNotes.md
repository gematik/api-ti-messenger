<img align="right" width="250" height="47" src="images/meta/gematik.png" alt="Gematik Logo"/>

# Release Notes api-ti-messenger

## tim-pro-1.0.0

### added

- TiMessengerInformation Rest interface
- Group exceptions in permissionConfig_V1 for TI-M Pro

### changed

- Separate permissionConfig_V1 for TI-M Pro & ePA

## Hotfix 1.1.1-9

### removed

- TiMessengerTestTreiber.yaml was moved to the Testsuite repository https://github.com/gematik/TI-Messenger-Testsuite under src/main/resources/api

## Hotfix 1.1.1-8

### changed

- changed FHIR status inactive to "off" to be FHIR compliant
- all properties required by the Testsuite marked as required in TiMessengerTestTreiber.yaml

## Hotfix 1.1.1-7

### changed

 - new tags in TiMessengerTestTreiber.yaml

## Hotfix 1.1.1-6

### fixed

- Fixed formatting issue in the api yaml, that prevented automated code generation.

## Hotfix 1.1.1-5

### added

- Additional examples and documentation for using the uri scheme of the matrix id (see https://spec.matrix.org/v1.3/appendices/#matrix-uri-scheme) in the FHIR directory. 

## Hotfix 1.1.1-4

### changed

- Testdriver api: Added roomVersion to room

## Hotfix 1.1.1-3

### changed

- Testdriver api: Added optional endpoint for clean up system
 
## Release 1.1.1 (2023-08-25)
 
### added
 
- initial version of the implementation guide for the ti-messenger-service according to the specification of the [TI-Messenger-Dienstes_v1.1.1](https://fachportal.gematik.de/schnelleinstieg/downloadcenter/releases#c7710)

<!-- possible examples for future versions 
### added
 
- added 1
- added 2
- added 3
 
### performance
 
- performance 1
 
## Release 1.0.0 (2022-02-11)
 
### changed
 
- change 1
- change 2
 
### fixed
 
- fixed 1
 
### security
 
- security 1 -->