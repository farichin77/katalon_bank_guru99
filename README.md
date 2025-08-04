# 🏦 Guru99 Bank Web Automation Testing (Katalon Studio)

📌 **Project Description**  
Project ini merupakan automation testing untuk website [Guru99 Bank Demo](https://demo.guru99.com/V4/) menggunakan **Katalon Studio**. Pengujian dilakukan terhadap fitur utama sistem perbankan seperti manajemen nasabah, transaksi keuangan, enquiry saldo, hingga penghapusan akun.

---

## ✅ Test Modules

🔹 Total terdapat **55 test cases** yang mencakup:

- Login & Logout
- Manajemen Customer (new, edit, delete)
- Manajemen Akun (new, edit, delete)
- Deposit & Withdrawal
- Fund Transfer
- Balance Enquiry
- Negative Testing (validasi input tidak sesuai)

Semua test case sudah dikelompokkan ke dalam beberapa folder modular di dalam struktur `Test Cases`.

---

## 🧪 Test Suites

Test case diatur dalam **5 Test Suites utama** untuk kemudahan pengelolaan:

| Test Suite             | Deskripsi                                                                 |
|------------------------|---------------------------------------------------------------------------|
| 01_SmokeTest           | Pengujian dasar untuk validasi akses dan navigasi                         |
| 02_customerManagement  | Fokus ke fitur Customer (create, edit, delete)                            |
| 03_accountManagement   | Menangani fitur akun                                                      |
| 04_NegatifTestCase     | Skenario pengujian input yang tidak valid                                 |
| 05_RegressionTest      | Menjalankan keseluruhan test untuk validasi sistem menyeluruh             |

---

## ⚙️ Automation Features

- ✅ Web UI automation dengan Katalon Studio
- ✅ GlobalVariable untuk share data antar test case
- ✅ Screenshot otomatis saat test gagal (via `Test Listener`)
- ✅ Organisasi modular berdasarkan fitur
- ✅ Dukungan untuk smoke test, regression, dan negative testing

---

## 🧰 Tools & Tech Stack

- **Katalon Studio**
- **Groovy / Gherkin**
- **Built-in Selenium WebDriver**
- **Custom Test Listener**
- **Test Suite Collection**

---

## 💾 Screenshot Evidence

📸 Screenshot otomatis diambil saat test gagal  
📁 Disimpan di direktori `/Screenshots/`  
🛠️ Diatur melalui file listener `NewTestListener.groovy`

---

## 📁 Folder Structure

Test Cases/
├── balance_enquiry/
├── delete_account/
├── delete_Customer/
├── deposit/
├── edit_account/
├── edit_customer/
├── Login_page/
├── logout/
├── new_account/
├── new_customer/
├── tranfer/
└── widraw/

Test Suites/
├── 01_SmokeTest
├── 02_customerManagement
├── 03_accountManagement
├── 04_NegatifTestCase
└── 05_RegressionTest

Test Listeners/
└── NewTestListener.groovy

yaml
Copy
Edit

---

## 👨‍💻 Author

**[Ahmad farichin]**  
Manual & Automation QA Engineer  
📧 ahmadfarichin98@gmail.com
