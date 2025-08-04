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

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.util.KeywordUtil

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.util.KeywordUtil
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webui.driver.DriverFactory
import org.openqa.selenium.WebDriver

WebUI.openBrowser('')
WebUI.navigateToUrl('https://demo.guru99.com/V4/')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_UserID_uid'), 'mngr629633')
WebUI.setEncryptedText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_Password_password'), 'gvzKTh1O0s0=')
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_Password_btnLogin'))

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Edit Customer'))
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Page/input_Customer ID_cusid'), '55043')
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Page/input_Customer ID_AccSubmit'))

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_E-mail_emailid'), 'agus@@gmail..com')
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_E-mail_sub'))

WebUI.delay(2)

// ✅ Ini bagian yang diperbaiki
WebDriver driver = DriverFactory.getWebDriver()
String pageSource = driver.getPageSource()

if (pageSource.trim().length() < 100) {
	KeywordUtil.markPassed("✅ Halaman kosong setelah submit email invalid")
} else {
	KeywordUtil.markFailed("❌ Halaman masih ada isi, email invalid tidak divalidasi dengan benar")
}

WebUI.closeBrowser()

