---
title: Frosh Handbook
subtitle: E-learning Platform
author: Kwatafana Systems
date: last update 29-01-2024
...
---

# Frosh

## FEATURES

- [ ] Website
- [ ] Cross platform Native App (Dart+Flutter)
- [ ] Student Registration
- [ ] Online classes & remote teaching
- [ ] Student Enquiries
- [ ] Student Attendance
- [ ] Announcements
- [ ] Examination Management
- [ ] Staff Management
- [ ] Timetabe Generation
- [ ] Cloud Based
- [ ] Books/ Library
- [ ] Parent Accounts
- [ ] Giving Credential (Degrees)
- [ ] Sport management
- Parents can check
  - [ ] Attendance
  - [ ] Homework
  - [ ] Exam results
  - [ ] Performance analytics
  - [ ] Email notifications

---

## Core Data Types

### Students

#### Students Schema Version 0

| Field              | Description                                                                    | Type             |
|--------------------|--------------------------------------------------------------------------------|------------------|
| __username__       | Students account username                                                      | `String`         |
| __firstname__      | Students first name                                                            | `String`         |
| __middlenames__    | Students middle names                                                          | `Option<String>` |
| __lastname__       | Students last name                                                             | `String`         |
| __email__          | Students email address                                                         | `String`         |
| __bio__            | Students bio                                                                   | `Option<String>` |
| __courses__        | Courses, it is u32 because it represents the course ID in the courses database | `Vec<u32>`       |
| __student_number__ | Student's number                                                               | `Option<String>` |
| __national_id__    | Student national ID number                                                     | `Option<String>` |
| __mobile__         | Students mobile phone number                                                   | `Option<String>` |
| __gender__         | Students gender                                                                | `Gender`         |
| __last_login__     | Last time the student logged into Frosh                                        | `DateTime<Utc>`  |
| __joined__         | Date when student joined institution                                           | `DateTime<Utc>`  |
| __version__        | Data type schema version                                                       | `u32`            |

---

## DATABASES

- [ ] Students.sqlite
- [ ] Teachers.sqlite
- [ ] Admins.sqlite
- [ ] Events.sqlite
- [ ] Library.sqlite
- [ ] Groups.sqlite
