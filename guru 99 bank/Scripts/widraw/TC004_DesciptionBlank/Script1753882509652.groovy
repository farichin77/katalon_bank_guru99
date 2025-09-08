import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.webui.common.WebUiCommonHelper as WebUiCommonHelper
import org.openqa.selenium.WebElement as WebElement
import java.util.Arrays as Arrays
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

CustomKeywords.'helpers.LoginHelper.login'()

// Klik Withdrawal via JavaScript
TestObject withdrawalLink = findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Withdrawal')

WebElement withdrawalElement = WebUiCommonHelper.findWebElement(withdrawalLink, 5)

WebUI.executeJavaScript('arguments[0].click();', Arrays.asList(withdrawalElement))

// Isi form withdraw
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Withdrawal Page/input_Account No_accountno'), '175238')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Withdrawal Page/input_Amount_ammount'), '30000')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Withdrawal Page/input_Description_desc'), '')

// Klik Submit via JavaScript
TestObject btnSubmit = findTestObject('Object Repository/Page_Guru99 Bank Amount Withdrawal Page/input_Description_AccSubmit')

WebElement submitElement = WebUiCommonHelper.findWebElement(btnSubmit, 5)

WebUI.executeJavaScript('arguments[0].click();', Arrays.asList(submitElement))

// Tambahkan delay agar alert sempat muncul
WebUI.delay(2)

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

