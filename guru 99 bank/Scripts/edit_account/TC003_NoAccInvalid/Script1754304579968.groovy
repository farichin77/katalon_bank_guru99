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
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper
import org.openqa.selenium.WebElement as WebElement
import java.util.Arrays as Arrays

WebUI.openBrowser('')

WebUI.navigateToUrl('https://demo.guru99.com/V4/')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_UserID_uid'))

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_UserID_uid'), 'mngr629633')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_Password_password'), 'gvzKTh1O0s0=')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_Password_btnLogin'))

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Edit Account'))

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Edit Account Page/input_Account No_accountno'), '151725672')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Edit Account Page/input_Account No_AccSubmit'))

// ❗ WAJIB HANDLE ALERT SEBELUM AKSI LAIN
if (WebUI.verifyAlertPresent(3, FailureHandling.OPTIONAL)) {
    String alertText = WebUI.getAlertText()
    WebUI.comment("⚠️ Alert muncul: " + alertText)

    WebUI.verifyMatch(alertText, 'Account does not exist', false)
    WebUI.acceptAlert()
    WebUI.comment('✅ Alert sudah ditutup dan sesuai ekspektasi')

    return // keluar dari test case, supaya tidak lanjut ke aksi DOM
} else {
    WebUI.comment("❌ Alert tidak muncul padahal expected")
    assert false
}