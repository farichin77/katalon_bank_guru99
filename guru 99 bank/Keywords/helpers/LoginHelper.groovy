package helpers

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

public class LoginHelper {

	@Keyword
	def login() {
		// Buka halaman login
		WebUI.openBrowser('')
		WebUI.maximizeWindow()
		WebUI.navigateToUrl('https://demo.guru99.com/V4/')

		// Isi username & password
		WebUI.setText(findTestObject('Object Repository/loginpage/input_UserID_uid'), GlobalVariable.username)
		WebUI.setText(findTestObject('Object Repository/loginpage/input_Password_password'), GlobalVariable.password)

		// Klik tombol login
		WebUI.click(findTestObject('Object Repository/loginpage/input_Password_btnLogin'))
	}
}
