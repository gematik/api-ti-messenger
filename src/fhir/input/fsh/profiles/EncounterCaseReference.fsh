Profile: EncounterCaseReference
Parent: Encounter
Description: "A profile of the Encounter resource for TI-Messenger communication related to a patients case reference."
* insert Meta
* subject 1..1 MS

// Instance that should pass validation
Instance: EncounterExample
InstanceOf: EncounterCaseReference
Description: "An example of a encounter."
Usage: #example
* id = "324534708"
* identifier.system = "http://example.de/StructureDefinition/identifier-interne-abrechnungsnummer"
* identifier.value = "ABC1234567890"
* status = #in-progress
* class.code = #AMB
* class.display = "ambulatory"
* priority.coding.code = #A
* priority.coding.display = "ASAP"
* subject.reference = "374885372" // reference to the patient resource
* period.start = "2022-05-02"
* reasonCode.coding.code = #368009
* reasonCode.coding.display = "Heart valve disorder"