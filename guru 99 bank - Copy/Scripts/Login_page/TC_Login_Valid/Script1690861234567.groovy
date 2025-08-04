import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

// Open browser and navigate to login page
WebUI.openBrowser('')
WebUI.navigateToUrl('http://demo.guru99.com/V4/')
WebUI.maximizeWindow()

// Input credentials
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_UserID'), GlobalVariable.USERNAME)
WebUI.setText(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_Password'), GlobalVariable.PASSWORD)

// Click login button
WebUI.click(findTestObject('Object Repository/Page_Guru99 Bank Home Page/input_LOGIN'))

// Verify successful login
WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/td_Manger Id'), 20)
WebUI.verifyElementText(findTestObject('Object Repository/Page_Guru99 Bank Manager HomePage/td_Manger Id'), 'Manger Id : ' + GlobalVariable.USERNAME)
