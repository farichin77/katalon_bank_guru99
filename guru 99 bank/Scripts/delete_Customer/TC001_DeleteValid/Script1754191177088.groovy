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

CustomKeywords.'helpers.LoginHelper.login'()

// Menu Delete Customer
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Delete Customer'))
if (!GlobalVariable.customerID || GlobalVariable.customerID.trim() == '') {
	WebUI.comment("❌ Customer ID belum tersedia. Jalankan test registrasi dulu.")
	assert false : "GlobalVariable.customerID kosong"
}


// Isi Customer ID
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Delete Customer Page/input_Customer ID_cusid'),GlobalVariable.customerID)

// Submit
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Delete Customer Page/input_Customer ID_AccSubmit'))

if (WebUI.waitForAlert(5)) {
    WebUI.acceptAlert()
    WebUI.delay(1)

    if (WebUI.waitForAlert(5)) {
        String alertHasil = WebUI.getAlertText()
        WebUI.comment("✅ Alert hasil: " + alertHasil)
        WebUI.acceptAlert()

        // 👉 Redirect ke home setelah alert sukses muncul
        WebUI.navigateToUrl('https://demo.guru99.com/V4/manager/Managerhomepage.php')
    } else {
        WebUI.comment("⚠️ Tidak ada alert hasil, mungkin halaman blank.")
        WebUI.takeScreenshot()

        // 👉 Redirect paksa kalau halaman blank
        WebUI.navigateToUrl('https://demo.guru99.com/V4/manager/Managerhomepage.php')
    }
} else {
    WebUI.comment("❌ Tidak ada konfirmasi alert.")
    WebUI.takeScreenshot()

    // Redirect paksa kalau gagal delete
    WebUI.navigateToUrl('https://demo.guru99.com/V4/manager/Managerhomepage.php')
}
