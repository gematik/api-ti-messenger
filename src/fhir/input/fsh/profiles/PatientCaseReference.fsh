Profile: PatientCaseReference
Parent: Patient
Description: "A profile of the Patient resource for TI-Messenger communication related to a patients case reference."
* insert Meta
* id MS
* identifier 1..* MS
  * ^slicing.discriminator.type = #value
  * ^slicing.discriminator.path = "system"
  * ^slicing.rules = #open
* identifier contains KVNR 1..
* identifier[KVNR].system = $IdentifierKVID10 (exactly)
  * ^patternIdentifier.type = $v2-0203#PRN
* identifier contains PKVID 0..
* identifier[PKVID].system = $IdentifierPKVID (exactly)
  * ^patternIdentifier.type = $v2-0203#PRN
* name 1..1 MS


// Instance that should pass validation
Instance: PatientExample
InstanceOf: PatientCaseReference
Description: "An example of a patient."
Usage: #example
* id = "374885372" // Meta id
* identifier.system = "http://fhir.de/StructureDefinition/identifier-kvid-10"
* identifier.value = "ABC1234567"
* name
  * given[0] = "August"
  * family = "Fröhlich"
* gender = #male
* birthDate = "2000-02-21"
* communication
  * language
    * coding
      * code = #de