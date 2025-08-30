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
import com.kms.katalon.core.util.KeywordUtil


CustomKeywords.'helpers.LoginHelper.login'()

CustomKeywords.'helpers.Navigationhelper.goToEditCustomerPage'()

CustomKeywords.'helpers.EditCustomerHelper.checkCustomer'('73367')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_E-mail_emailid'), '')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_Email-ID must not be blank_sub'))

// Tunggu alert muncul
if (WebUI.waitForAlert(3)) {
	String alertText = WebUI.getAlertText()
	if (alertText.contains("Email-ID must not be blank")) {
		KeywordUtil.markPassed("✅ Validasi email is blank berhasil: " + alertText)
	} else {
		KeywordUtil.markFailed("⚠️ seharusnya submit tidak bisa di klik jika email kosong: " + alertText)
	}
	WebUI.acceptAlert()
} else {
	KeywordUtil.markFailed("❌ Tidak ada alert, kemungkinan email blank masih bisa submit")
}


