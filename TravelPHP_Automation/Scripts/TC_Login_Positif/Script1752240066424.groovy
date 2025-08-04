import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

// 1. Buka browser
WebUI.openBrowser('')
WebUI.maximizeWindow()

// 2. Akses halaman login
WebUI.navigateToUrl('https://phptravels.net/login')

// 3. Hilangkan cookie popup atau overlay yang ganggu klik
WebUI.delay(2)
WebUI.executeJavaScript("""
    let cookie = document.getElementById('cookieconsent:desc');
    if (cookie) cookie.style.display = 'none';

    let fadeout = document.querySelector('.fadeout');
    if (fadeout) fadeout.style.display = 'none';
""", null)

// 4. Isi email
WebUI.setText(findTestObject('Page_Login/input_Email Address_email'), 'user@phptravels.com')

// 5. Isi password
WebUI.setEncryptedText(findTestObject('Page_Login/input_Password_password'), '6weEYLVRi3+STmppai9GiQ==')

// 6. Klik checkbox pakai JS biar aman (tanpa intersep)
WebUI.executeJavaScript("""
    let cb = document.getElementById('rememberchb');
    if (cb) cb.click();
""", null)

// 7. Matikan elemen yang nutupin tombol login
WebUI.executeJavaScript("""
    var cookieDisclaimer = document.getElementById('cookie_disclaimer');
    if (cookieDisclaimer) {
        cookieDisclaimer.style.display = 'none';
    }
""", null)


// 8. Klik tombol Login
WebUI.click(findTestObject('Page_Login/button_Login'))

// 9. Tunggu halaman dashboard tampil
WebUI.waitForPageLoad(10)
WebUI.delay(2)

// 10. Verifikasi login sukses
boolean isLoginSuccess = WebUI.verifyElementPresent(findTestObject('Page_Login/div_WelcomeMessage'), 10, FailureHandling.OPTIONAL)

if (isLoginSuccess) {
    println("✅ Login berhasil")
} else {
    println("❌ Login gagal")
}

