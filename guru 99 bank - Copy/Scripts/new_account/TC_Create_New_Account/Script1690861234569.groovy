import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

// Click on New Account menu
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/link_New Account'))

// Fill in account details
WebUI.setText(findTestObject('Object Repository/Page_Guru99 bank add new account/input_Customer ID'), GlobalVariable.CUSTOMER_ID)
WebUI.selectOptionByValue(findTestObject('Object Repository/Page_Guru99 bank add new account/select_Account Type'), 'Savings', false)
WebUI.setText(findTestObject('Object Repository/Page_Guru99 bank add new account/input_Initial Deposit'), '1000')

// Submit form
WebUI.click(findTestObject('Object Repository/Page_Guru99 bank add new account/input_Submit'))

// Verify successful account creation
WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Guru99 Bank Account Generated/td_Account Generated Successfully'), 20)

// Store account ID for future use
GlobalVariable.ACCOUNT_ID = WebUI.getText(findTestObject('Object Repository/Page_Guru99 Bank Account Generated/td_Account ID'))
