package helpers

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import com.kms.katalon.core.testobject.ObjectRepository as OR

import internal.GlobalVariable

public class formCustomerhelper {

	@Keyword
	def fillNewCustomerForm(Map data) {

		// Customer Name
		TestObject nameObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_Customer Name_name')
		WebUI.waitForElementVisible(nameObj, 5)
		WebUI.setText(nameObj, data.name ?: '')

		// Gender
		TestObject genderObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/td_malefemale')
		WebUI.waitForElementVisible(genderObj, 5)
		WebUI.click(genderObj)

		// DOB
		TestObject dobObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_DOB')
		WebUI.waitForElementVisible(dobObj, 5)
		WebUI.setText(dobObj, data.dob ?: '')

		// Address
		TestObject addressObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/textarea_Address_addr')
		WebUI.waitForElementVisible(addressObj, 5)
		WebUI.setText(addressObj, data.address ?: '')

		// City
		TestObject cityObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_City_city')
		WebUI.waitForElementVisible(cityObj, 5)
		WebUI.setText(cityObj, data.city ?: '')

		// State
		TestObject stateObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_State_state')
		WebUI.waitForElementVisible(stateObj, 5)
		WebUI.setText(stateObj, data.state ?: '')

		// PIN
		TestObject pinObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_PIN_pinno')
		WebUI.waitForElementVisible(pinObj, 5)
		WebUI.setText(pinObj, data.pin ?: '')

		// Mobile Number
		TestObject mobileObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_Mobile Number_telephoneno')
		WebUI.waitForElementVisible(mobileObj, 5)
		WebUI.setText(mobileObj, data.mobile ?: '')

		// Email
		TestObject emailObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_E-mail_emailid')
WebUI.waitForElementVisible(emailObj, 5)

String emailToUse = data.email
if (!emailToUse) {
    String timestamp = new Date().getTime().toString()
    emailToUse = 'andi' + timestamp + '@mail.com'
}

WebUI.setText(emailObj, emailToUse)

		// Password
		TestObject passwordObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_Password_password')
		WebUI.waitForElementVisible(passwordObj, 5)
		WebUI.setText(passwordObj, data.password ?: '')
	}

	@Keyword
	def submitNewCustomerForm() {
		TestObject submitObj = findTestObject('Page_Guru99 Bank Home Page/Page_Guru99 Bank New Customer Entry Page/input_Password_sub')
		WebUI.waitForElementVisible(submitObj, 5)
		WebUI.click(submitObj)
	}

	@Keyword
	def goHome() {
		TestObject homeObj = findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank Customer Registration Page/a_Home')
		WebUI.waitForElementVisible(homeObj, 5)
		WebUI.click(homeObj)
	}
}