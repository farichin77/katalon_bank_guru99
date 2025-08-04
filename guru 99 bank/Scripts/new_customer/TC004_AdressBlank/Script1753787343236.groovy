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

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://demo.guru99.com/V4/')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank Home Page/input_UserID_uid'), 
    'mngr629633')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank Home Page/input_Password_password'), 
    'gvzKTh1O0s0=')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank Home Page/input_Password_btnLogin'))

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank Manager HomePage/a_New Customer'))

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_Customer Name_name'), 
    'agus')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/td_malefemale'))

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_DOB'), 
    '02-02-1999')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/textarea_Address_addr'), 
    '')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_City_city'), 
    'bekasi')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_State_state'), 
    'jawa barat')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_PIN_pinno'), 
    '123456')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_Mobile Number_telephoneno'), 
    '123456789')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/td_PIN Code must have 6 Digits'))

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_E-mail_emailid'), 
    'agus123@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_Password_password'), 
    'iS/zEYUBSn/RLxP4zhxx2Q==')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_Password_sub'))

// Cek jika alert muncul (expected behavior)
if (WebUI.verifyAlertPresent(2, FailureHandling.OPTIONAL)) {
    String alertText = WebUI.getAlertText()

    WebUI.comment('⚠️ Alert muncul: ' + alertText)

    // Verifikasi isi alert sesuai yang diharapkan
    WebUI.verifyMatch(alertText.toLowerCase(), 'please fill all fields', false)

    WebUI.acceptAlert()

    WebUI.comment('✅ Test PASSED karena alert muncul sesuai expected')
} else {
    WebUI.comment('❌ Test FAILED: Tidak ada alert yang muncul padahal seharusnya ada!')

    assert false
}

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank Customer Registration Page/a_Home'))

WebUI.closeBrowser()

