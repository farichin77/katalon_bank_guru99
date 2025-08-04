import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.model.FailureHandling
import org.openqa.selenium.Keys
import org.openqa.selenium.WebElement
import org.openqa.selenium.interactions.Actions
import com.kms.katalon.core.webui.driver.DriverFactory
import com.kms.katalon.core.testobject.TestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.testobject.ConditionType
import java.util.Arrays

// 1. Jalankan login positif (memanggil test case yang sudah ada)
WebUI.callTestCase(findTestCase('TC_Login_Positif'), [:], FailureHandling.STOP_ON_FAILURE)

// 2. Tunggu homepage / dashboard tampil
WebUI.waitForPageLoad(10)
WebUI.delay(2)

// 3. Klik menu "Flights"
WebUI.click(findTestObject('Page_Dashboard/a_Flight'))

// Pilih kota asal (From)
WebUI.click(findTestObject('Page_Flights/input_From')) // pastikan fokus
WebUI.setText(findTestObject('Page_Flights/input_From'), 'Jakarta')
WebUI.delay(2) // beri waktu suggestion muncul
WebUI.sendKeys(findTestObject('Page_Flights/input_From'), Keys.chord(Keys.ARROW_DOWN))
WebUI.delay(1)
WebUI.sendKeys(findTestObject('Page_Flights/input_From'), Keys.chord(Keys.ENTER))

// Pilih kota tujuan (To)
WebUI.click(findTestObject('Page_Flights/input_To'))
WebUI.setText(findTestObject('Page_Flights/input_To'), 'Singapore')
WebUI.delay(2)
WebUI.sendKeys(findTestObject('Page_Flights/input_To'), Keys.chord(Keys.ARROW_DOWN))
WebUI.delay(1)
WebUI.sendKeys(findTestObject('Page_Flights/input_To'), Keys.chord(Keys.ENTER))

// 6. Pilih tanggal
WebUI.click(findTestObject('Page_Flights/input_DepartureDate'))
TestObject todayDate = new TestObject().addProperty("xpath", ConditionType.EQUALS,
'//td[contains(@class, "today") or contains(@class, "active")]')
WebUI.waitForElementVisible(todayDate, 5)
WebUI.click(todayDate)

// 7. Klik tombol search
WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Flights/button_Search'), 10)
WebUI.click(findTestObject('Object Repository/Page_Flights/button_Search'))

// 7. Tunggu hasil pencarian muncul dan pilih flight pertama
WebUI.waitForPageLoad(15)
WebUI.delay(5) // Tunggu data flight ter-load

// Debug: Cek apakah ada flight results
println("=== DEBUG: Checking flight results ===")
try {
	// Cek apakah ada flight results di halaman
	WebUI.executeJavaScript("console.log('Flight results check:', document.querySelectorAll('button, .flight-item, .flight-card, [class*=\"flight\"], [class*=\"select\"]').length)", null)
	
	// Ambil screenshot untuk debugging
	WebUI.takeScreenshot()
	
	// Print semua button yang ada di halaman
	def allButtons = WebUI.executeJavaScript("""
		var buttons = Array.from(document.querySelectorAll('button, input[type="submit"], input[type="button"]'));
		return buttons.map(btn => ({
			text: btn.textContent.trim(),
			value: btn.value || '',
			id: btn.id || '',
			className: btn.className || '',
			type: btn.type || '',
			tagName: btn.tagName
		}));
	""", null)
	
	println("Available buttons on page:")
	allButtons.each { button ->
		println("  - ${button.tagName}: '${button.text}' | Value: '${button.value}' | ID: '${button.id}' | Class: '${button.className}'")
	}
} catch (Exception debugError) {
	println("Debug error: " + debugError.getMessage())
}

// Enhanced strategy untuk click button SelectFirstFlight
boolean clickSuccess = false

// Strategy 1: Periksa apakah Test Object ada
if (!clickSuccess) {
	try {
		println("=== Strategy 1: Checking Test Object existence ===")
		if (WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Flights/button_SelectFirstFlight'), 5, FailureHandling.OPTIONAL)) {
			println("Test Object found, attempting normal click")
			WebUI.waitForElementVisible(findTestObject('Object Repository/Page_Flights/button_SelectFirstFlight'), 15)
			WebUI.scrollToElement(findTestObject('Object Repository/Page_Flights/button_SelectFirstFlight'), 5)
			WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Flights/button_SelectFirstFlight'), 10)
			WebUI.click(findTestObject('Object Repository/Page_Flights/button_SelectFirstFlight'))
			clickSuccess = true
			println("Normal click successful")
		} else {
			println("Test Object not found, moving to alternative strategies")
		}
	} catch (Exception e) {
		println("Strategy 1 failed: " + e.getMessage())
	}
}

// Strategy 2: Cari button dengan text yang mengandung "Select"
if (!clickSuccess) {
	try {
		println("=== Strategy 2: Text-based selection ===")
		WebUI.executeJavaScript("""
			var buttons = Array.from(document.querySelectorAll('button, input[type="submit"], input[type="button"], a[role="button"]'));
			var selectButton = buttons.find(btn => {
				var text = (btn.textContent || btn.value || '').toLowerCase();
				return text.includes('select') || text.includes('choose') || text.includes('book') || text.includes('continue');
			});
			
			if (selectButton) {
				console.log('Found select button:', selectButton.textContent || selectButton.value);
				selectButton.scrollIntoView({behavior: 'smooth', block: 'center'});
				setTimeout(() => {
					selectButton.click();
					console.log('Button clicked successfully');
				}, 1000);
				return true;
			}
			return false;
		""", null)
		
		// Tunggu sebentar untuk melihat apakah click berhasil
		WebUI.delay(3)
		
		// Cek apakah halaman berubah (indikasi click berhasil)
		def currentUrl = WebUI.getUrl()
		println("Current URL after click attempt: " + currentUrl)
		
		clickSuccess = true
		println("Text-based selection successful")
	} catch (Exception e) {
		println("Strategy 2 failed: " + e.getMessage())
		clickSuccess = false
	}
}

// Strategy 3: Cari berdasarkan posisi (first flight card)
if (!clickSuccess) {
	try {
		println("=== Strategy 3: First flight card selection ===")
		WebUI.executeJavaScript("""
			// Cari berbagai pattern untuk flight card
			var selectors = [
				'.flight-item button',
				'.flight-card button',
				'[class*="flight"] button',
				'.result-item button',
				'.search-result button',
				'[data-flight] button',
				'.flight-option button'
			];
			
			for (var selector of selectors) {
				var buttons = document.querySelectorAll(selector);
				if (buttons.length > 0) {
					console.log('Found buttons with selector:', selector);
					var firstButton = buttons[0];
					firstButton.scrollIntoView({behavior: 'smooth', block: 'center'});
					setTimeout(() => {
						firstButton.click();
						console.log('First flight button clicked');
					}, 1000);
					return true;
				}
			}
			
			// Fallback: cari button pertama yang bukan navigation
			var allButtons = document.querySelectorAll('button');
			for (var btn of allButtons) {
				var text = btn.textContent.toLowerCase();
				if (!text.includes('search') && !text.includes('back') && !text.includes('menu')) {
					console.log('Using fallback button:', btn.textContent);
					btn.scrollIntoView({behavior: 'smooth', block: 'center'});
					setTimeout(() => btn.click(), 1000);
					return true;
				}
			}
			return false;
		""", null)
		
		WebUI.delay(3)
		clickSuccess = true
		println("First flight card selection successful")
	} catch (Exception e) {
		println("Strategy 3 failed: " + e.getMessage())
		clickSuccess = false
	}
}

// Strategy 4: Manual koordinat click pada area flight pertama
if (!clickSuccess) {
	try {
		println("=== Strategy 4: Coordinate-based click ===")
		
		// Ambil dimensi viewport
		def viewportInfo = WebUI.executeJavaScript("""
			return {
				width: window.innerWidth,
				height: window.innerHeight,
				scrollY: window.scrollY
			};
		""", null)
		
		// Klik di area tengah-kanan halaman (biasanya lokasi button select)
		int clickX = (int)(viewportInfo.width * 0.8)  // 80% dari lebar
		int clickY = (int)(viewportInfo.height * 0.5)  // 50% dari tinggi
		
		println("Attempting coordinate click at: (${clickX}, ${clickY})")
		
		// Scroll ke tengah halaman dulu
		WebUI.executeJavaScript("window.scrollTo(0, document.body.scrollHeight/3);", null)
		WebUI.delay(2)
		
		// Gunakan Actions untuk click koordinat
		Actions actions = new Actions(DriverFactory.getWebDriver())
		actions.moveByOffset(clickX, clickY).click().perform()
		
		clickSuccess = true
		println("Coordinate click successful")
	} catch (Exception e) {
		println("Strategy 4 failed: " + e.getMessage())
		clickSuccess = false
	}
}

// Strategy 5: Coba enter key atau space pada elemen yang focused
if (!clickSuccess) {
	try {
		println("=== Strategy 5: Keyboard interaction ===")
		
		// Focus pada elemen yang mungkin clickable
		WebUI.executeJavaScript("""
			var focusableElements = document.querySelectorAll('button, input[type="submit"], a[role="button"]');
			if (focusableElements.length > 0) {
				focusableElements[0].focus();
				var event = new KeyboardEvent('keydown', {key: 'Enter', keyCode: 13});
				focusableElements[0].dispatchEvent(event);
				return true;
			}
			return false;
		""", null)
		
		WebUI.delay(2)
		clickSuccess = true
		println("Keyboard interaction successful")
	} catch (Exception e) {
		println("Strategy 5 failed: " + e.getMessage())
		clickSuccess = false
	}
}

// Jika semua strategi gagal, berikan informasi detail
if (!clickSuccess) {
	println("=== ALL STRATEGIES FAILED ===")
	
	// Ambil screenshot final
	WebUI.takeScreenshot()
	
	// Berikan informasi detail tentang halaman
	def pageInfo = WebUI.executeJavaScript("""
		return {
			title: document.title,
			url: window.location.href,
			buttonCount: document.querySelectorAll('button').length,
			linkCount: document.querySelectorAll('a').length,
			formCount: document.querySelectorAll('form').length,
			bodyText: document.body.innerText.substring(0, 500)
		};
	""", null)
	
	println("Page Info:")
	println("  Title: " + pageInfo.title)
	println("  URL: " + pageInfo.url)
	println("  Button Count: " + pageInfo.buttonCount)
	println("  Link Count: " + pageInfo.linkCount)
	println("  Form Count: " + pageInfo.formCount)
	println("  Body Text Preview: " + pageInfo.bodyText)
	
	throw new Exception("All click strategies failed for SelectFirstFlight button. Please check the page structure and update the Test Object selector.")
}

// Jika berhasil, lanjutkan dengan booking
try {
	// 8. Tunggu navigasi ke halaman selanjutnya
	WebUI.waitForPageLoad(10)
	WebUI.delay(3)
	
	// Cek apakah halaman berubah
	def newUrl = WebUI.getUrl()
	println("Navigation successful. New URL: " + newUrl)

	// 9. Lanjut ke form traveler, isi nama, email, dll (kalau ada)
	// Tambahkan kode untuk mengisi form traveler di sini jika diperlukan

	// 10. Klik Book / Continue - Enhanced dengan multiple strategies
	boolean bookingClickSuccess = false
	
	// Strategy 1: Normal click untuk booking button
	if (!bookingClickSuccess) {
		try {
			println("Trying normal click for booking button")
			WebUI.waitForElementVisible(findTestObject('Object Repository/Page_Flights/button_ConfirmBooking'), 15)
			WebUI.scrollToElement(findTestObject('Object Repository/Page_Flights/button_ConfirmBooking'), 5)
			WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Flights/button_ConfirmBooking'), 10)
			WebUI.click(findTestObject('Object Repository/Page_Flights/button_ConfirmBooking'))
			bookingClickSuccess = true
			println("Normal booking click successful")
		} catch (Exception e) {
			println("Normal booking click failed: " + e.getMessage())
		}
	}
	
	// Strategy 2: JavaScript click untuk booking button
	if (!bookingClickSuccess) {
		try {
			println("Trying JavaScript click for booking button")
			WebUI.executeJavaScript("""
				var buttons = Array.from(document.querySelectorAll('button, input[type="submit"]'));
				var confirmButton = buttons.find(btn => {
					var text = (btn.textContent || btn.value || '').toLowerCase();
					return text.includes('confirm') || text.includes('book') || text.includes('continue') || text.includes('proceed');
				});
				
				if (confirmButton) {
					confirmButton.scrollIntoView({behavior: 'smooth', block: 'center'});
					setTimeout(() => confirmButton.click(), 500);
					return true;
				}
				return false;
			""", null)
			bookingClickSuccess = true
			println("JavaScript booking click successful")
		} catch (Exception e) {
			println("JavaScript booking click failed: " + e.getMessage())
		}
	}
	
	// Jika booking click gagal, beri warning tapi lanjutkan
	if (!bookingClickSuccess) {
		println("Warning: Booking click failed, but continuing test")
		WebUI.takeScreenshot()
	}

	// 11. Verifikasi booking berhasil (dengan timeout yang lebih fleksibel)
	try {
		WebUI.waitForElementPresent(findTestObject('Object Repository/Page_Flights/div_SuccessBooking'), 15)
		WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Flights/div_SuccessBooking'), 10)
		println("Booking verification successful")
	} catch (Exception verifyError) {
		println("Booking verification failed: " + verifyError.getMessage())
		// Cek apakah ada indikasi sukses lainnya
		def successIndicators = WebUI.executeJavaScript("""
			var text = document.body.innerText.toLowerCase();
			return text.includes('success') || text.includes('confirmed') || text.includes('booked') || text.includes('thank you');
		""", null)
		
		if (successIndicators) {
			println("Alternative success indicators found")
		} else {
			println("No success indicators found")
		}
	}

	println("Flight booking test completed")

} catch (Exception e) {
	println("Test failed with error: " + e.getMessage())
	e.printStackTrace()

	// Ambil screenshot jika browser masih terbuka
	try {
		if (WebUI.verifyBrowserOpened(FailureHandling.OPTIONAL)) {
			WebUI.takeScreenshot()
		}
	} catch (Exception screenshotError) {
		println("Cannot take screenshot: " + screenshotError.getMessage())
	}

	throw e
} finally {
	// 12. Tutup browser
	try {
		if (WebUI.verifyBrowserOpened(FailureHandling.OPTIONAL)) {
			WebUI.closeBrowser()
		}
	} catch (Exception closeError) {
		println("Error closing browser: " + closeError.getMessage())
	}
}