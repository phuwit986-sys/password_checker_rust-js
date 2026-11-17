# Password Checker (Node.js & Rust) & การตรวจสอบความปลอดภัยด้วย Semgrep

เอกสารนี้รวบรวม **วิธีติดตั้ง**, **วิธีรัน**, **วิธีสแกนด้วย Semgrep**, **ตารางกรณีทดสอบ (Test Cases)** พร้อมผลที่คาดหวังและผลจริง รวมถึง **สรุปผลการสแกน Semgrep และคำอธิบายความปลอดภัย** สำหรับโปรแกรมตรวจสอบรหัสผ่านทั้งเวอร์ชัน Node.js (`password_checker.js`) และ Rust (`main.rs`)

---

## 1. โครงสร้างโปรเจกต์ (Project Structure)

```text
rust&js/
├── password_checker.js       # ตัวตรวจจับรหัสผ่านเวอร์ชัน JavaScript (Node.js)
├── main.rs                   # ซอร์สโค้ด Rust (Standalone / Backup)
├── README.md                 # คู่มือการติดตั้ง รัน ทดสอบ และสแกน Semgrep
└── password_checker_rust/    # โปรเจกต์ Rust (Cargo Project)
    ├── Cargo.toml            # กำหนด Package และ Dependency (crossterm)
    ├── Cargo.lock
    ├── src/
    │   └── main.rs           # โค้ดหลักของเวอร์ชัน Rust
    └── target/
        └── release/
            └── password_checker_rust.exe  # ไบนารีพร้อมรัน
```

---

## 2. วิธีการติดตั้ง (Installation Guide)

### 2.1 สิ่งที่ต้องเตรียม (Prerequisites)
1. **Node.js**: เวอร์ชัน 18+ (เครื่องนี้ติดตั้ง `v26.13.1`)
2. **Rust & Cargo**: เวอร์ชัน 1.70+ (เครื่องนี้ติดตั้ง `cargo 1.100.0`)
3. **Python & Pip**: สำหรับติดตั้ง Semgrep CLI (ติดตั้ง Python 3.14 / 3.15)

---

### 2.2 การติดตั้ง Dependency

#### สำหรับ Node.js
ไฟล์ `password_checker.js` ใช้ Built-in Module (`readline`, `process`) เท่านั้น ไม่ต้องลง npm package เพิ่มเติม:
```bash
# ตรวจสอบเวอร์ชัน Node.js
node -v
```

#### สำหรับ Rust
โปรเจกต์ใช้ Dependency `crossterm = "0.27"` ในการจับ Key Event และจัดการ Raw Mode เพื่อ Mask รหัสผ่าน:
```bash
cd password_checker_rust
cargo build --release
```

---

### 2.3 การติดตั้ง Semgrep บน Windows
Semgrep เป็นเครื่องมือ Static Application Security Testing (SAST) สามารถติดตั้งผ่าน `pip` ได้ดังนี้:

```powershell
# ติดตั้ง Semgrep ผ่าน Python pip
pip install semgrep

# หรือเจาะจงพาธ Python (ตัวอย่าง)
& "C:\Users\admin\AppData\Local\Programs\Python\Python314\python.exe" -m pip install semgrep

# ตรวจสอบการติดตั้ง
semgrep --version
```

> [!NOTE]
> หากเรียกคำสั่ง `semgrep` แล้วแจ้งเตือนว่าไม่พบคำสั่ง (Command Not Found) ให้เพิ่มพาธ `Scripts` ของ Python เข้าไปใน Environment Variables (System/User PATH) เช่น:
> `C:\Users\<User>\AppData\Local\Programs\Python\Python314\Scripts`

---

## 3. วิธีการรันโปรแกรม (How to Run)

> [!IMPORTANT]
> ทั้งสองโปรแกรมทำงานใน **Terminal Raw Mode** เพื่อซ่อนการพิมพ์รหัสผ่านเป็นเครื่องหมาย `*` จึงต้องรันผ่าน Interactive Console หรือ PowerShell / Command Prompt โดยตรง (ไม่สามารถส่งค่าผ่าน stdin pipe ได้)

### 3.1 รันเวอร์ชัน JavaScript (Node.js)
```powershell
# รันไฟล์ JavaScript จากโฟลเดอร์ rust&js
node password_checker.js
```

### 3.2 รันเวอร์ชัน Rust
สามารถรันผ่าน Cargo หรือรันไฟล์ `.exe` ที่คอมไพล์แล้ว:
```powershell
# วิธีที่ 1: รันผ่าน Cargo
cd password_checker_rust
cargo run --release

# วิธีที่ 2: รันไฟล์ Executable โดยตรง
.\target\release\password_checker_rust.exe
```

---

## 4. เงื่อนไขและกฎของโปรแกรม (Validation Rules)

1. **โควตากรอกข้อมูล**: ให้โอกาสกรอกข้อมูลได้สูงสุด **3 ครั้ง** หากไม่ผ่านครบ 3 ครั้งจะแจ้ง `ผิดครบ 3 ครั้ง Logout!` และออกจากโปรแกรม
2. **Username (Email)**: ต้องอยู่ในรูปแบบอีเมลที่มีเครื่องหมาย `@` และมีชื่อโดเมนพร้อมจุด `.` เช่น `user@example.com` โดยไม่มีช่องว่าง
3. **Password Masking**: ขณะพิมพ์รหัสผ่าน หน้าจอจะแสดงผลเป็นเครื่องหมาย `*` รองรับการลบด้วย `Backspace` และตัดปุ่มควบคุมพิเศษ
4. **ความยาวรหัสผ่าน (Password Length)**: ต้องมีความยาว **อย่างน้อย 15 ตัวอักษร (`MIN_LEN = 15`)** โดยนับตามจำนวนอักขระ Unicode จริง (ไม่ใช่ byte length)

---

## 5. ตารางกรณีทดสอบ (Test Cases Table)

| รหัสทดสอบ (TC ID) | กรณีทดสอบ (Test Scenario) | ข้อมูลนำเข้า: Username | ข้อมูลนำเข้า: Password | ผลลัพธ์ที่คาดหวัง (Expected Result) | ผลลัพธ์จากการรันจริง (Actual Result) | สถานะ (Status) |
|---|---|---|---|---|---|---|
| **TC-01** | ผ่านสมบูรณ์: Username รูปแบบอีเมลถูกต้อง และ Password ยาวเกิน 15 ตัวอักษร | `student@mail.com` | `SafeP@ssword2026!` *(17 ตัวอักษร)* | - รหัสผ่านถูก Mask ด้วย `*`<br>- แสดง: `ผ่าน: ความยาว 17 ตัวอักษร`<br>- แสดง: `Welcome student@mail.com`<br>- จบโปรแกรมทันที | - รหัสผ่านขึ้น `*****************`<br>- แสดง: `ผ่าน: ความยาว 17 ตัวอักษร`<br>- แสดง: `Welcome student@mail.com`<br>- จบโปรแกรมเรียบร้อย | **PASS** |
| **TC-02** | ผ่านขอบล่าง (Boundary Value): Password มีความยาวพอดี 15 ตัวอักษร | `admin@security.org` | `123456789012345` *(15 ตัวอักษร)* | - แสดง: `ผ่าน: ความยาว 15 ตัวอักษร`<br>- แสดง: `Welcome admin@security.org` | - แสดง: `ผ่าน: ความยาว 15 ตัวอักษร`<br>- แสดง: `Welcome admin@security.org` | **PASS** |
| **TC-03** | ไม่ผ่าน (Email): Username ไม่มีเครื่องหมาย `@` | `admin_user` | `P@ssword12345678` *(16 ตัวอักษร)* | - แสดง: `ไม่ผ่าน (1/3): username ต้องเป็นอีเมล`<br>- วนให้กรอกใหม่รอบที่ 2 | - แสดง: `ไม่ผ่าน (1/3): username ต้องเป็นอีเมล`<br>- วนรอบเข้าสู่ครั้งที่ 2 | **PASS** |
| **TC-04** | ไม่ผ่าน (Email): มี `@` แต่ไม่มีชื่อโดเมน/จุด (`.`) | `user@localhost` | `P@ssword12345678` *(16 ตัวอักษร)* | - แสดง: `ไม่ผ่าน (1/3): username ต้องเป็นอีเมล`<br>- วนให้กรอกใหม่รอบที่ 2 | - แสดง: `ไม่ผ่าน (1/3): username ต้องเป็นอีเมล`<br>- วนรอบเข้าสู่ครั้งที่ 2 | **PASS** |
| **TC-05** | ไม่ผ่าน (Password สั้น): Password ยาว 14 ตัวอักษร (Boundary Value: MIN_LEN - 1) | `dev@domain.co.th` | `P@ssword123456` *(14 ตัวอักษร)* | - แสดง: `ไม่ผ่าน (1/3): รหัสผ่านสั้นเกินไป (14/15 ตัวอักษร)`<br>- วนให้กรอกใหม่ | - แสดง: `ไม่ผ่าน (1/3): รหัสผ่านสั้นเกินไป (14/15 ตัวอักษร)`<br>- วนให้กรอกใหม่ | **PASS** |
| **TC-06** | ทดสอบการนับ Unicode / Emoji ในรหัสผ่าน | `tester@app.io` | `Pass🛡️Key🔑123456` | - นับตัวอักษรตาม Code Point จริง ไม่เกิดข้อผิดพลาดนับ Byte ซ้ำ | - ทั้ง JS (`[...pass].length`) และ Rust (`chars().count()`) นับได้ถูกต้อง ไม่ผิดเพี้ยน | **PASS** |
| **TC-07** | ทดสอบฟังก์ชันปุ่ม Backspace ในการ Mask รหัสผ่าน | `test@example.com` | พิมพ์ตัวอักษรแล้วกด Backspace ลบ 2 ครั้ง แล้วพิมพ์ต่อจนครบ | - เครื่องหมาย `*` บนหน้าจอลดลงตามการกดลบ<br>- ตัวแปร Password ในหน่วยความจำตัดอักขระออกจริง | - หน้าจอแสดง `\b \b` ลบดาวถูกต้อง<br>- รหัสผ่านคำนวณความยาวตรงตามค่าสุดท้าย | **PASS** |
| **TC-08** | ทดสอบโควตาความล้มเหลวครบ 3 ครั้ง (Account Lockout) | กรอกผิดทั้ง 3 ครั้ง เช่น username ไม่เป็นอีเมล หรือ password สั้น | กรอกผิดทั้ง 3 ครั้ง | - แจ้งเตือนข้อผิดพลาดรอบที่ (1/3), (2/3), (3/3)<br>- แสดง: `ผิดครบ 3 ครั้ง Logout!`<br>- โปรแกรมยุติการทำงาน | - แจ้งเตือนตามลำดับรอบ<br>- แสดง: `ผิดครบ 3 ครั้ง Logout!`<br>- สิ้นสุดโปรแกรมทันที | **PASS** |

---

## 6. วิธีการสแกนด้วย Semgrep (How to Scan with Semgrep)

### 6.1 คำสั่งสแกนอัตโนมัติ (Default / Auto Scan)
คำสั่งนี้จะค้นหากฎมาตรฐานของ Semgrep Registry ที่เหมาะสมกับโปรเจกต์โดยอัตโนมัติ:
```powershell
semgrep scan --config auto
```

### 6.2 คำสั่งสแกนแบบเจาะจง Security Audit & OWASP
สามารถระบุ Ruleset ด้านความปลอดภัยโดยเฉพาะได้ดังนี้:
```powershell
# สแกนเฉพาะชุด Security Audit และ Secret Detection
semgrep scan --config "p/security-audit" --config "p/secrets"

# สแกนเฉพาะชุด OWASP Top Ten
semgrep scan --config "p/owasp-top-ten"
```

### 6.3 คำสั่ง Export รายงานผลการสแกนเป็น JSON หรือ Text
```powershell
# บันทึกผลเป็น Text Report
semgrep scan --config auto > semgrep_report.txt

# บันทึกผลเป็น JSON Format สำหรับ CI/CD Pipeline
semgrep scan --config auto --json -o semgrep_report.json
```

---

## 7. สรุปผลการสแกน Semgrep และคำอธิบาย (Semgrep Results & Analysis)

### 7.1 ผลการรัน Semgrep จริง (Actual Scan Output)

จากการรันคำสั่ง `semgrep scan --config auto` บนโปรเจกต์ ได้ผลลัพธ์ดังนี้:

```text
┌─────────────┐
│ Scan Status │
└─────────────┘
  Scanning 12 files tracked by git with 1074 Code rules:

  Language      Rules   Files          Origin      Rules
 ─────────────────────────────        ───────────────────
  <multilang>      47      12          Community    1074
  rust              4       2
  js              153       1
  json              4       1

┌──────────────┐
│ Scan Summary │
└──────────────┘
✅ Scan completed successfully.
 • Findings: 0 (0 blocking)
 • Rules run: 206
 • Targets scanned: 12
 • Parsed lines: ~100.0%
 • Scan skipped:
   ◦ Files larger than 1.0 MB: 1 (Executable / Build target)
   ◦ Files matching .semgrepignore patterns: 13
Ran 206 rules on 12 files: 0 findings.
```

---

### 7.2 คำอธิบายผลลัพธ์และการวิเคราะห์เชิงความปลอดภัย (Security Analysis)

#### 1. เหตุผลที่ผลลัพธ์เป็น 0 Findings (Clean Scan)
1. **ไม่มีการใช้ Dynamic Code Execution หรือ Injection Sink**: ทั้งโค้ด JS และ Rust ไม่มีการใช้งานคำสั่งอันตราย เช่น `eval()`, `Function()`, `child_process.exec()`, หรือการต่อ String เข้ากับ SQL / Shell Commands
2. **ไม่มีการบันทึก Hardcoded Secrets**: โค้ดรับรหัสผ่านจาก Input ของผู้ใช้งานแบบไดนามิก ไม่มีการฝังรหัสผ่านหรือ API Keys ทิ้งไว้ใน Source Code
3. **การจัดการ Terminal ปลอดภัย**: มีการจัดการคืนสถานะ Terminal ปิด Raw Mode หลังจากการรับค่าเสร็จสิ้น

---

#### 2. ข้อพิจารณาความปลอดภัยเพิ่มเติมที่ Static Analysis (SAST) ทั่วไปอาจมองข้าม (Deep Security Audit)

แม้ว่า Semgrep จะไม่พบช่องโหว่ระดับ High/Critical แต่จากการตรวจสอบโค้ดแบบ Manual Code Review มีประเด็นความปลอดภัยที่ควรทราบและนำไปปรับปรุงดังนี้:

| ประเด็น (Topic) | ข้อสังเกตในโค้ดปัจจุบัน | ข้อเสนอแนะเพื่อยกระดับความปลอดภัย (Recommendations) |
|---|---|---|
| **การจัดเก็บรหัสผ่านใน Memory** | ทั้งใน JS (`chars = []`, `String`) และ Rust (`String`) รหัสผ่านจะถูกจัดเก็บในหน่วยความจำแบบ Cleartext ซึ่งใน JavaScript ตัวแปร String เป็น Immutable ไม่สามารถ Overwrite เมมโมรีทิ้งได้ทันที ต้องรอ Garbage Collection | ในระบบระดับ Production ที่รองรับความปลอดภัยสูง ควรใช้เทคนิคการเคลียร์ Buffer เช่น Crate `zeroize` ใน Rust เพื่อล้างข้อมูลรหัสผ่านออกจาก RAM ทันทีหลังตรวจสอบเสร็จ |
| **นโยบายความซับซ้อนของรหัสผ่าน (Password Complexity)** | โปรแกรมตรวจสอบเฉพาะ **ความยาวขั้นต่ำ 15 ตัวอักษร** (`MIN_LEN >= 15`) แต่ไม่ได้บังคับส่วนผสม เช่น ตัวพิมพ์ใหญ่-เล็ก ตัวเลข หรืออักขระพิเศษ | ควรเพิ่ม Regex หรือเงื่อนไขตรวจสอบ Character Sets (Uppercase, Lowercase, Digits, Symbols) เพื่อป้องกันการตั้งรหัสผ่านยาวแต่ซ้ำกันง่าย เช่น `aaaaaaaaaaaaaaa` |
| **การจำกัดจำนวนครั้ง (Rate Limiting / Lockout)** | มีการจำกัด 3 ครั้งเฉพาะ **ภายใน Session การทำงานปัจจุบัน** เท่านั้น เมื่อสั่งรันโปรแกรมใหม่ ผู้ใช้จะสามารถเริ่มนับ 1 ใหม่ได้ทันที | หากเป็นระบบจริง ควรมีการบันทึกประวัติการ Login ผิดลงใน Database หรือ Cache (เช่น Redis) พร้อมบันทึก Timestamp เพื่อ Lock บัญชีชั่วคราว |
| **การจัดการ Signal / Uncaught Exception ใน Raw Mode** | หากโปรแกรมปิดตัวลงอย่างกะทันหันก่อนหลุดจาก Raw Mode อาจส่งผลให้ Terminal ของผู้ใช้ค้างหรือไม่แสดงตัวอักษร | ควรเพิ่ม Exception Handler / Drop Guard / Signal Handler (SIGINT, SIGTERM) เพื่อการันตีว่า `terminal::disable_raw_mode()` หรือ `stdin.setRawMode(false)` จะถูกเรียกเสมอ |
| **ความครอบคลุมของ Semgrep Rule ต่อภาษา Rust** | Semgrep Community Ruleset มีกฎสำหรับ JavaScript (153 กฎ) มากกว่า Rust (4 กฎ) | สำหรับภาษา Rust ควรพิจารณาใช้เครื่องมือเฉพาะทางร่วมด้วย เช่น `cargo audit` (ตรวจสอบช่องโหว่ใน Dependency) และ `cargo clippy` (ตรวจสอบคุณภาพและมาตรฐานโค้ด) |

---
*จัดทำขึ้นสำหรับการทดสอบระบบ Password Checker และการประเมินความปลอดภัยด้วย Semgrep*
