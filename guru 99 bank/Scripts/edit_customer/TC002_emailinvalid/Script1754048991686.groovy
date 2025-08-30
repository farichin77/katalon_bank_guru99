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

CustomKeywords.'helpers.LoginHelper.login'()

CustomKeywords.'helpers.Navigationhelper.goToEditCustomerPage'()

CustomKeywords.'helpers.EditCustomerHelper.checkCustomer'('73367')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_E-mail_emailid'), 'agus@@gmail..com')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_E-mail_sub'))

WebUI.delay(2)

// Tunggu alert muncul
if (WebUI.waitForAlert(3)) {
    String alertText = WebUI.getAlertText()
    if (alertText.contains("Email-ID is not valid")) {
        KeywordUtil.markPassed("✅ Validasi email invalid berhasil: " + alertText)
    } else {
        KeywordUtil.markFailed("⚠️ Alert muncul tapi bukan validasi email: " + alertText)
    }
    WebUI.acceptAlert()
} else {
    KeywordUtil.markFailed("❌ Tidak ada alert, kemungkinan email invalid masih bisa submit")
}



