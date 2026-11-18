**BRCO Process Automation System**

**1. Introduction**
The Process Automation System (PAS) is an automated system implemented for business units where manual repetitive work is still prevalent. The PAS is intended to
automate repetitive administrative, compliance, and operational control processes within a corporate banking environment.

They system is intended to reduce manual processing, improve data consistency, strengthen internal controls, and provide an auditable framework for monitoring operational
activities.


**2. Objectives**

The primary objectives of PAS are:
1. **Process Automation:** reduce manual intervention in recurring administrative and control activities.

2. **Data Integrity:** validate information across forms, documents, and system-generated records while reducing human error that occur during manual inputs.

3. **Operational Efficiency:** streamline the workflow of the Business Management Unit and easing the internal control testing process.

4. **Centralised Recordkeeping:** maintain consistent, uniquely identifiable records of submissions, processing outcomes, and exceptions.

5. **Auditability:** Establish traceable records of transactions, validation outcomes, system actions, and authorised overrides.

PAS is designed to **assist** staff in performing their responsibilities rather than replace established approval authorities, professional judgment, or human oversight.





**3. System Architecture**

PAS is designed to be modular, separating workflo orchestration, business logic, persistent storage, and external integrations. It is intended to be implemented and embedded
within core Microsoft Office systems currently being used. 

PAS is divided into five principle layers with clear assignment of responsibilities:

**a) Workflow:** Scheduling, form collections, notifications, and orchestration through Microsoft Power Automate

**b) API:** Receiving requests, returning processing results, and exposing defined system operations'

**c) Business Logic:** Validation, credit proposal registration, routing, control execution, and exception detection.

**d) Persistence:** Maintaining proposal records, counters, exceptions, transaction history, and audit information.

**e) Integration:** Connecting to document repositories, banking data sources, and reporting destinations.

**System Architecture**
