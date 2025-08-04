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
import com.kms.katalon.core.webui.driver.DriverFactory as DriverFactory
import org.openqa.selenium.WebDriver as WebDriver
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://demo.guru99.com/V4/')

WebUI.setText(findTestObject('Page_Guru99 Bank Home Page/input_UserID_uid'), 'mngr629633')

WebUI.setEncryptedText(findTestObject('Page_Guru99 Bank Home Page/input_Password_password'), 'gvzKTh1O0s0=')

WebUI.click(findTestObject('Page_Guru99 Bank Home Page/input_Password_btnLogin'))

WebUI.click(findTestObject('Page_Guru99 Bank Manager HomePage/a_Edit Customer'))

WebUI.setText(findTestObject('Page_Guru99 Bank Edit Customer Page/input_Customer ID_cusid'), GlobalVariable.customerID)

WebUI.click(findTestObject('Page_Guru99 Bank Edit Customer Page/input_Customer ID_AccSubmit'))

WebUI.setText(findTestObject('Page_Guru99 Bank Edit Customer Entry Page/textarea_Address'), 'jalan cemara rt 12 rw 09 bekasi selatan kota bekas')

WebUI.setText(findTestObject('Page_Guru99 Bank Edit Customer Entry Page/input_City_city'), 'kota bekasi')

WebUI.setText(findTestObject('Page_Guru99 Bank Edit Customer Entry Page/input_State_state'), 'jawa barat')

WebUI.setText(findTestObject('Page_Guru99 Bank Edit Customer Entry Page/input_PIN_pinno'), '123456')

WebUI.setText(findTestObject('Page_Guru99 Bank Edit Customer Entry Page/input_Mobile Number_telephoneno'), '0876435678')

WebUI.setText(findTestObject('Page_Guru99 Bank Edit Customer Entry Page/input_E-mail_emailid'), 'agus123@gmail.com')

TestObject submitButton = findTestObject('Page_Guru99 Bank Edit Customer Entry Page/input_E-mail_sub')

// 🔁 Fokus keluar dari input terakhir (biar gak auto scroll lagi)
WebUI.executeJavaScript('document.activeElement.blur();', null)

// ⬆️ Scroll ke atas paksa
WebUI.executeJavaScript('window.scrollTo(0, 0);', null)

WebUI.delay(1)

// 🎯 Scroll ke tombol + tunggu agar stabil
WebUI.scrollToElement(submitButton, 5)

WebUI.waitForElementClickable(submitButton, 10)

// 🖱️ Klik tombol dengan fallback ke JS jika perlu
try {
    WebUI.click(submitButton)
}
catch (Exception e) {
    WebUI.comment('⚠️ Klik biasa gagal, pakai JS click.')

    WebElement element = WebUiCommonHelper.findWebElement(submitButton, 10)

    WebUI.executeJavaScript('arguments[0].click();', Arrays.asList(element))
} 

