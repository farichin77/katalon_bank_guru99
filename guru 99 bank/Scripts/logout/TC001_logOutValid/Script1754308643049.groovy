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
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper

WebUI.openBrowser('')
WebUI.maximizeWindow()
WebUI.navigateToUrl('https://demo.guru99.com/V4/')

WebUI.setText(findTestObject('Page_Guru99 Bank Home Page/input_UserID_uid'), 'mngr629633')
WebUI.setEncryptedText(findTestObject('Page_Guru99 Bank Home Page/input_Password_password'), 'gvzKTh1O0s0=')
WebUI.click(findTestObject('Page_Guru99 Bank Home Page/input_Password_btnLogin'))

// Sembunyikan elemen copyright & iklan Zoho
WebUI.executeJavaScript("let el1 = document.querySelector('p[style*=\"text-align:center\"]'); if(el1){ el1.style.display='none'; }", null)
WebUI.executeJavaScript("let img = document.querySelector('img[src*=\"Zoho\"]'); if(img){ let parent = img.closest('div'); if(parent){ parent.style.display='none'; }}", null)
WebUI.delay(1)

TestObject logoutBtn = findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Log out')
WebElement element = WebUiCommonHelper.findWebElement(logoutBtn, 10)
WebUI.executeJavaScript("arguments[0].click();", Arrays.asList(element))

// Terima alert konfirmasi logout
if (WebUI.waitForAlert(5)) {
    WebUI.acceptAlert()
    WebUI.comment("✅ Logout berhasil & alert diterima")
} else {
    WebUI.comment("⚠️ Tidak ada alert muncul saat logout")
}

WebUI.closeBrowser()
