Profile: EncounterTIMessengerEncounter
Parent: Encounter
Description: "A profile of the Encounter resource for TI-Messenger communication related to an encounter with a patient."
* insert Meta
* id MS
* identifier MS
* status MS
* statusHistory MS
* class MS
* classHistory MS
* serviceType MS
* priority MS
* subject 1..1 MS
* period MS
* length MS
* reasonCode MS
* diagnosis MS

// Instance that should pass validation
Instance: EncounterExample
InstanceOf: EncounterTIMessengerEncounter
Description: "An example of a encounter."
Usage: #example
* id = "324534708"
* identifier.system = "http://example.de/StructureDefinition/identifier-interne-abrechnungsnummer"
* identifier.value = "ABC1234567890"
* status = #in-progress
* class.code = #AMB
* class.display = "ambulatory"
* priority.code = #A
* priority.display = "ASAP"
* subject = "374885372" // reference to the patient resource
* period.start = "2022-05-02"
* reasonCode.code = #368009
* reasonCode.display = "Heart valve disorder"