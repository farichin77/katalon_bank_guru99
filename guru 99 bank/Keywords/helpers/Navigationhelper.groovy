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

import internal.GlobalVariable

public class Navigationhelper {

	@Keyword
	def goToNewCustomerPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/Page_Guru99 Bank Manager HomePage/a_New Customer'))
	}

	@Keyword
	def goToEditCustomerPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Edit Customer'))
	}

	@Keyword
	def goToNewAccountPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_New Account'))
	}

	@Keyword
	def goToEditAccountPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Edit Account'))
	}

	@Keyword
	def goToDepositPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Deposit'))
	}

	@Keyword
	def goToWithdrawPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Withdrawal'))
	}

	@Keyword
	def goToFundTransferPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Fund Transfer'))
	}

	@Keyword
	def goToBalanceEnquiryPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Balance Enquiry'))
	}

	@Keyword
	def goToMiniStatementPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Mini Statement'))
	}

	@Keyword
	def goToCustomisedStatementPage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/a_Customised Statement'))
	}

	@Keyword
	def goToHomePage() {
		WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Customer Registration Page/a_Home'))
	}
}
