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

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_UserID_uid'), 'mngr629633')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_Password_password'), 'obZSWqv3hcI=')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_Password_btnLogin'))

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Deposit'))

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Deposit Page/input_Account No_accountno'), '')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Deposit Page/input_Amount_ammount'), '100000')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Deposit Page/input_Description_desc'), 'nabung')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Amount Deposit Page/input_Description can not be blank_AccSubmit'))

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

WebUI.closeBrowser()

