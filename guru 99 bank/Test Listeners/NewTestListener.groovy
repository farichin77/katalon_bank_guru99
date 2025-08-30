import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.BeforeTestCase
import com.kms.katalon.core.annotation.BeforeTestSuite
import com.kms.katalon.core.annotation.AfterTestCase
import com.kms.katalon.core.annotation.AfterTestSuite
import com.kms.katalon.core.context.TestCaseContext
import com.kms.katalon.core.context.TestSuiteContext

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webui.driver.DriverFactory

class NewTestListener {
	
	@AfterTestCase
    def takeScreenshotOnFailure(TestCaseContext testCaseContext) {
        try {
            if (DriverFactory.getWebDriver() != null &&
                testCaseContext.getTestCaseStatus() == 'FAILED') {

                String testCaseName = testCaseContext.getTestCaseId().tokenize('/').last()
                String timeStamp = new Date().format("yyyyMMdd_HHmmss")
                String screenshotPath = RunConfiguration.getProjectDir() + "/Screenshots/${testCaseName}_${timeStamp}.png"

                WebUI.takeScreenshot(screenshotPath)
                KeywordUtil.logInfo("📸 Screenshot saved at: " + screenshotPath)
            }
        } catch (Exception e) {
            KeywordUtil.markWarning("⚠️ Gagal ambil screenshot: " + e.message)
        }
    }
}