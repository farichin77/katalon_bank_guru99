import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.apache.commons.lang.RandomStringUtils

// Click on New Customer menu
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/link_New Customer'))

// Generate random customer data
def customerName = 'Test Customer ' + RandomStringUtils.randomAlphabetic(5)
def dateOfBirth = '01012000'
def address = 'Test Address ' + RandomStringUtils.randomAlphanumeric(10)
def city = 'Test City ' + RandomStringUtils.randomAlphabetic(5)
def state = 'Test State ' + RandomStringUtils.randomAlphabetic(5)
def pin = RandomStringUtils.randomNumeric(6)
def mobile = '123' + RandomStringUtils.randomNumeric(7)
def email = 'test' + RandomStringUtils.randomAlphanumeric(5).toLowerCase() + '@test.com'
def password = 'Test@' + RandomStringUtils.randomAlphanumeric(6)

// Fill in customer details
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_Customer Name'), customerName)
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_Gender_Male'))
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_Date of Birth'), dateOfBirth)
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/textarea_Address'), address)
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_City'), city)
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_State'), state)
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_PIN'), pin)
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_Mobile Number'), mobile)
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_E-mail'), email)
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_Password'), password)

// Submit form
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank New Customer Entry Page/input_Submit'))

// Verify successful customer creation
WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Guru99 Bank Customer Registration Page/td_Customer Registered Successfully'), 20)

// Store customer ID for future use
GlobalVariable.CUSTOMER_ID = WebUI.getText(findTestObject('Object Repository/Page_Guru99 Bank Customer Registration Page/td_Customer ID'))
