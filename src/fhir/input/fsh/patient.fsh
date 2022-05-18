Profile: PatientTIMessengerEncounter
Parent: Patient
Description: "A profile of the Patient resource for TI-Messenger communication related to an encounter with a patient."
* name 1..* MS

Instance: PatientExample
InstanceOf: MyPatient
Description: "An example of a patient with a license to krill."
* name
  * given[0] = "James"
  * family = "Pond"