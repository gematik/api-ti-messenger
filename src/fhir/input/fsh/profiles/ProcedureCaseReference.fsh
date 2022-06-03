Profile: ProcedureCaseReference
Parent: Procedure
Description: "A profile of the Procedure resource for TI-Messenger communication related to a patients case reference."
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
Instance: ProcedureExample
InstanceOf: ProcedureCaseReference
Description: "An example of a encounter."
Usage: #example
* identifier.system = "http://example.de/StructureDefinition/identifier-interne-abrechnungsnummer"
* identifier.value = "ABC1234567890"
* status = #preparation
* subject.reference = "374885372"
* encounter.reference = "324534708"
* note.text = "Additional information"