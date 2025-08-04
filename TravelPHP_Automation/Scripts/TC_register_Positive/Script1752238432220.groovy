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

WebUI.closeBrowser()

WebUI.openBrowser('')

WebUI.navigateToUrl('https://phptravels.com/demo')

WebUI.click(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/div_Demo            Live Demonstration     _91007f_1'))

WebUI.setText(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/input_Request Demo Access_first_name'), 
    'ahmad')

WebUI.setText(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/input_First Name_last_name'), 
    'farichin')

WebUI.setText(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/input_Select Country_whatsapp'), 
    '89624687381')

WebUI.setText(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/input_WhatsApp Number_business_name'), 
    'far')

WebUI.setText(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/input_Business Name_email'), 'ahmadfarichin11@gmail.com')

WebUI.setText(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/input__number'), '12')

WebUI.click(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/button_Submit'))

WebUI.click(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/button_Submit'))

WebUI.navigateToUrl('https://phptravels.com/demo')

WebUI.closeBrowser()

WebUI.setText(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/input__number'), '3')

WebUI.click(findTestObject('Object Repository/Page_Book Your Free Demo Now - Phptravels/div_Submit'))

WebUI.navigateToUrl('https://phptravels.com/demo')

