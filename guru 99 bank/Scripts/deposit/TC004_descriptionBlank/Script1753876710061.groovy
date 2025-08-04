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

// Buka browser
WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl('https://demo.guru99.com/V4/')

// Login
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_UserID_uid'), 'mngr629633')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_Password_password'), 'gvzKTh1O0s0=')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_Password_btnLogin'))

// Ke halaman Deposit
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Deposit'))

// Isi form dengan Description kosong
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Deposit Page/input_Account No_accountno'), '175238')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Deposit Page/input_Amount_ammount'), '50000')

WebUI.clearText(findTestObject('Object Repository/Page_Guru99 Bank Amount Deposit Page/input_Description_desc'))

// Tunggu form ready
WebUI.delay(1)

// Paksa klik tombol submit via JavaScript
TestObject btnSubmit = findTestObject('Object Repository/Page_Guru99 Bank Amount Deposit Page/input_Description can not be blank_AccSubmit')

WebElement element = WebUiCommonHelper.findWebElement(btnSubmit, 5)

WebUI.executeJavaScript('arguments[0].click();', Arrays.asList(element))

// Tunggu alert muncul
WebUI.delay(2)

if (WebUI.verifyAlertPresent(5, FailureHandling.OPTIONAL)) {
    String alertText = WebUI.getAlertText()

    WebUI.comment('⚠️ Alert muncul: ' + alertText)

    WebUI.verifyMatch(alertText.toLowerCase(), 'please fill all fields', false)

    WebUI.acceptAlert()

    WebUI.comment('✅ Test PASSED: Alert sesuai expectation')
} else {
    WebUI.comment('❌ Test FAILED: Tidak ada alert yang muncul meskipun seharusnya ada!')

    assert false
}

WebUI.closeBrowser()

