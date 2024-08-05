#[doc = "Register `APP_CM4_CTI_CONTROL` reader"]
pub type R = crate::R<AppCm4CtiControlSpec>;
#[doc = "Register `APP_CM4_CTI_CONTROL` writer"]
pub type W = crate::W<AppCm4CtiControlSpec>;
#[doc = "Field `APP_CM4_CTI_CONTROL` reader - 31:0\\]
http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDGDIHE.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDHBDIA.html"]
pub type AppCm4CtiControlR = crate::FieldReader<u32>;
#[doc = "Field `APP_CM4_CTI_CONTROL` writer - 31:0\\]
http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDGDIHE.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDHBDIA.html"]
pub type AppCm4CtiControlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDGDIHE.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDHBDIA.html"]
    #[inline(always)]
    pub fn app_cm4_cti_control(&self) -> AppCm4CtiControlR {
        AppCm4CtiControlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDGDIHE.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDHBDIA.html"]
    #[inline(always)]
    #[must_use]
    pub fn app_cm4_cti_control(&mut self) -> AppCm4CtiControlW<AppCm4CtiControlSpec> {
        AppCm4CtiControlW::new(self, 0)
    }
}
#[doc = "http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html\n\nYou can [`read`](crate::Reg::read) this register and get [`app_cm4_cti_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`app_cm4_cti_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppCm4CtiControlSpec;
impl crate::RegisterSpec for AppCm4CtiControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_cm4_cti_control::R`](R) reader structure"]
impl crate::Readable for AppCm4CtiControlSpec {}
#[doc = "`write(|w| ..)` method takes [`app_cm4_cti_control::W`](W) writer structure"]
impl crate::Writable for AppCm4CtiControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_CM4_CTI_CONTROL to value 0"]
impl crate::Resettable for AppCm4CtiControlSpec {
    const RESET_VALUE: u32 = 0;
}
