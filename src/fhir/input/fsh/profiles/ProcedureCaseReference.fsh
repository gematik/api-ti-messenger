Profile: ProcedureTIMessengerEncounter
Parent: Procedure
Description: "A profile of the Procedure resource for TI-Messenger communication related to an encounter with a patient."
* insert Meta
* id MS
* identifier MS
* status MS
* statusReason MS
* category MS
* code MS
* reasonCode MS
* outcome MS
* complication MS
* subject MS
* encounter MS
* note MS

// Instance that should pass validation
/*
Instance: ProcedureExample
InstanceOf: ProcedureTIMessengerEncounter
Description: "An example of a encounter."
Usage: #example
* identifier.system = "http://example.de/StructureDefinition/identifier-interne-abrechnungsnummer"
* identifier.value = "ABC1234567890"
* encounter = "324534708"
* subject = "374885372"
* note.text = "Additional information"
*/