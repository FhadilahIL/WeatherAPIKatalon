<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Test case for Forecast 5 Days of Jakarta</description>
   <name>Forecast5Day</name>
   <tag></tag>
   <elementGuidId>b2172dc8-dc84-454b-bf90-c559cd7bc497</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>10.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://api.openweathermap.org/data/2.5/forecast?q=${city_name}&amp;units=${units}&amp;appid=${app_id}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.app_id</defaultValue>
      <description></description>
      <id>3f9038b0-d1dd-44f1-9f13-81ff7e392c0c</id>
      <masked>false</masked>
      <name>app_id</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.city_name</defaultValue>
      <description></description>
      <id>182fccb4-2010-47c8-ab65-fb1d44b0a494</id>
      <masked>false</masked>
      <name>city_name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.units</defaultValue>
      <description></description>
      <id>9b249a6e-0bb0-4423-993f-7e7a8ae0ff94</id>
      <masked>false</masked>
      <name>units</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import Weathe

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
