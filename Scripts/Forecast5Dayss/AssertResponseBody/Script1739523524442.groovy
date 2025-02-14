import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.Paths
import java.security.Timestamp
import java.time.LocalDateTime

import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.ResponseObject as ResponseObject
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

// Sending Request to Object Forecast5Day
ResponseObject res = WS.sendRequest(findTestObject('Forecast5Day'))

// Assert Response cod is equal 200
assert WS.verifyElementPropertyValue(res, 'cod', 200)

// Instance Date
Date now = new Date()

// Create Dir for JSON (in dir Documents/JSON)
Path jsonDirectory = Paths.get(System.getProperty('user.home')).resolve('Documents').resolve('JSON').resolve('Forecast 5 Days')

Files.createDirectories(jsonDirectory)

// Create txt File for Forecast 5 Days API Response
Path file = jsonDirectory.resolve('Forecast5Days_'+ now.getTime() +'.txt')

// Fill the txt File with Response
file.append(res.getResponseBodyContent())