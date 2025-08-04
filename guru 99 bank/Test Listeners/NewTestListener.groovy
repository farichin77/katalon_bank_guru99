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

	@BeforeTestSuite
	def beforeTestSuite(TestSuiteContext testSuiteContext) {
		// Buat folder Screenshots jika belum ada
		String screenshotDir = RunConfiguration.getProjectDir() + "/Screenshots"
		new File(screenshotDir).mkdirs()
		KeywordUtil.logInfo("📁 Folder Screenshots disiapkan di: " + screenshotDir)
	}

	@AfterTestCase
	def afterTestCase(TestCaseContext testCaseContext) {
		try {
			// Cek apakah browser terbuka
			if (DriverFactory.getWebDriver() != null) {
				// Ambil nama test case terakhir (hanya nama, bukan path lengkap)
				String testCaseName = testCaseContext.getTestCaseId().tokenize('/').last()
				String timeStamp = new Date().format("yyyyMMdd_HHmmss")
				String screenshotPath = RunConfiguration.getProjectDir() + "/Screenshots/${testCaseName}_${timeStamp}.png"

				WebUI.takeScreenshot(screenshotPath)
				KeywordUtil.logInfo("📸 Screenshot saved at: " + screenshotPath)
				// Baru tutup browser setelah screenshot
				WebUI.closeBrowser()
			} else {
				KeywordUtil.logInfo("❗ Browser tidak terbuka saat afterTestCase, screenshot dilewati.")
			}
		} catch (Exception e) {
			KeywordUtil.markWarning("⚠️ Gagal ambil screenshot: " + e.message)
		}
	}

	// Tambahan jika ingin pakai setup suite atau lainnya
	@BeforeTestCase
	def beforeTestCase(TestCaseContext testCaseContext) {
		KeywordUtil.logInfo("▶️ Starting test case: " + testCaseContext.getTestCaseId())
	}

	@AfterTestSuite
	def afterTestSuite(TestSuiteContext testSuiteContext) {
		KeywordUtil.logInfo("✅ Test suite selesai: " + testSuiteContext.getTestSuiteId())
	}
}
