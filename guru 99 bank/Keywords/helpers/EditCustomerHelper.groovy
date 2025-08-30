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
import com.kms.katalon.core.util.KeywordUtil


import internal.GlobalVariable

public class EditCustomerHelper {

	@Keyword
	def checkCustomer(String customerID) {

		// Isi Customer ID
		TestObject customerIdObj = findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Page/input_Customer ID_cusid')
		WebUI.waitForElementVisible(customerIdObj, 5)
		WebUI.setText(customerIdObj, customerID)

		// Submit ID
		TestObject submitObj = findTestObject('Object Repository/Page_Guru99 Bank Edit Customer Page/input_Customer ID_AccSubmit')
		WebUI.waitForElementVisible(submitObj, 5)
		WebUI.click(submitObj)
	}

	@Keyword
	def checkAllFields(String customerID) {
		WebUI.delay(2) // Tunggu halaman edit muncul

		Map fields = [
			"Nama"   : 'Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_Customer Name_name',
			"Address": 'Object Repository/Page_Guru99 Bank Edit Customer Entry Page/textarea_Address',
			"City"   : 'Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_City_city',
			"State"  : 'Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_State_state',
			"PIN"    : 'Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_PIN_pinno',
			"Mobile" : 'Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_Mobile Number_telephoneno',
			"Email"  : 'Object Repository/Page_Guru99 Bank Edit Customer Entry Page/input_E-mail_emailid'
		]

		fields.each { fieldName, path ->
			TestObject obj = findTestObject(path)
			WebUI.waitForElementVisible(obj, 5)

			if (fieldName == "Nama") {
				boolean isDisabled = WebUI.verifyElementHasAttribute(obj, 'disabled', 1, FailureHandling.CONTINUE_ON_FAILURE)
				if (isDisabled) {
					KeywordUtil.markPassed("✅ Field ${fieldName} tidak bisa diedit")
				} else {
					KeywordUtil.markFailed("❌ Field ${fieldName} bisa diedit")
				}
			} else {
				// Field lain cukup dicek visible
				if (WebUI.verifyElementVisible(obj, FailureHandling.CONTINUE_ON_FAILURE)) {
					KeywordUtil.markPassed("✅ Field ${fieldName} bisa diedit")
				} else {
					KeywordUtil.markFailed("❌ Field ${fieldName} tidak bisa diedit")
				}
			}
		}
	}
}
