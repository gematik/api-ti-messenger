Profile: ConditionCaseReference
Parent: Condition
Description: "A profile of the Condition resource for TI-Messenger communication related to a patients case reference"
* insert Meta
* id MS
* identifier MS
* clinicalStatus MS
* verificationStatus MS
* category MS
* severity MS
* code MS
* bodySite MS
* subject 1..1 MS
* encounter MS
* recordedDate MS
* stage MS
* evidence MS
* note MS

// Instance that should pass validation
Instance: ConditionExample
InstanceOf: ConditionCaseReference
Description: "An example of a condition."
Usage: #example
* identifier.system = "http://example.de/StructureDefinition/identifier-interne-abrechnungsnummer"
* identifier.value = "ABC1234567890"
* clinicalStatus.coding.code  = #active
* verificationStatus.coding.code = #provisional
* category.coding.code = #encounter-diagnosis
* severity.coding.code = #24484000
* severity.coding.display = "Severe"
* code.coding.code = #368009
* code.coding.display = "Heart valve disorder"
* subject.reference = "374885372"
* encounter.reference = "324534708"
* recordedDate = "2022-05-02"
* note.text = "Additional information"