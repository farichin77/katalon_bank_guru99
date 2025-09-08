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

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_New Account'))

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Amount Withdrawal Page/Page_Guru99 bank add new account/body_window.dataLayer  window.dataLayer    _1692b1'))

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Withdrawal Page/Page_Guru99 bank add new account/input_Customer id_cusid'), '55043')

WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Amount Withdrawal Page/Page_Guru99 bank add new account/input_Initial deposit_inideposit'), '40000')

WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Amount Withdrawal Page/Page_Guru99 bank add new account/input_Initial deposit_button2'))

WebUI.click(findTestObject('Object Repository/Page_Bank99 Customer Registration Page/a_Home'))

WebUI.closeBrowser()

