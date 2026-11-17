**BRCO Process Automation System**

**1. Introduction**
The Process Automation System (PAS) is an automated system implemented for the Business Management Unit, Corporate Banking Department, Baiduri Bank. The PAS is intended to
automate repetitive administrative, compliance, and operational control processes within a corporate banking environment.

They system is intended to reduce manual processing, improve data consistency, strengthen internal controls, and provide an auditable framework for monitoring operational
activities.


**2. Goals**

**Phase 1: Automated Credit Proposal Submission Register**



**3. System Architecture**

PAS is designed to be modular, separating workflo orchestration, business logic, persistent storage, and external integrations. It is intended to be implemented and embedded
within the core Microsoft Office systems currently being used. 

PAS is divided into five principle layers with clear assignment of responsibilities:

**a) Workflow:** Scheduling, form collections, notifications, and orchestration through Microsoft Power Automate
**b) API:** Receiving requests, returning processing results, and exposing defined system operations
**c) Business Logic:** Validation, credit proposal registration, routing, control execution, and exception detection.
**d) Persistence:** Maintaining proposal records, counters, exceptions, transaction history, and audit information.
**e) Integration:** Connecting to document repositories, banking data sources, and reporting destinations.

**System Architecture**
