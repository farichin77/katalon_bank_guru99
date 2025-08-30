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

CustomKeywords.'helpers.Navigationhelper.goToNewCustomerPage'()

CustomKeywords.'helpers.formCustomerhelper.fillNewCustomerForm'([('name') : 'andi jaya', ('gender') : 'Male', ('dob') : '01/01/1999'
        , ('address') : 'ds mulyosari rt 12 rw 02 bekasi', ('city') : 'Bekasi', ('state') : 'Jawa Barat', ('pin') : ''
        , ('mobile') : '123456789', ('email') : '', ('password') : 'iS/zEYUBSn/RLxP4zhxx2Q=='])

CustomKeywords.'helpers.formCustomerhelper.submitNewCustomerForm'()

CustomKeywords.'helpers.AlertHelper.verifyAlertMessage'('please fill all fields')

CustomKeywords.'helpers.formCustomerhelper.goHome'()

WebUI.closeBrowser()

