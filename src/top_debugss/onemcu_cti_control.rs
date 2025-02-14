#[doc = "Register `ONEMCU_CTI_CONTROL` reader"]
pub type R = crate::R<OnemcuCtiControlSpec>;
#[doc = "Register `ONEMCU_CTI_CONTROL` writer"]
pub type W = crate::W<OnemcuCtiControlSpec>;
#[doc = "Field `ONEMCU_CTI_CONTROL` reader - 31:0\\]
http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDGDIHE.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDHBDIA.html"]
pub type OnemcuCtiControlR = crate::FieldReader<u32>;
#[doc = "Field `ONEMCU_CTI_CONTROL` writer - 31:0\\]
http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDGDIHE.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDHBDIA.html"]
pub type OnemcuCtiControlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDGDIHE.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDHBDIA.html"]
    #[inline(always)]
    pub fn onemcu_cti_control(&self) -> OnemcuCtiControlR {
        OnemcuCtiControlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDGDIHE.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0480e/CHDHBDIA.html"]
    #[inline(always)]
    #[must_use]
    pub fn onemcu_cti_control(&mut self) -> OnemcuCtiControlW<OnemcuCtiControlSpec> {
        OnemcuCtiControlW::new(self, 0)
    }
}
#[doc = "http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdjefbi.html http://infocenter.arm.com/help/topic/com.arm.doc.ddi0314h/Chdefejc.html\n\nYou can [`read`](crate::Reg::read) this register and get [`onemcu_cti_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`onemcu_cti_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OnemcuCtiControlSpec;
impl crate::RegisterSpec for OnemcuCtiControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`onemcu_cti_control::R`](R) reader structure"]
impl crate::Readable for OnemcuCtiControlSpec {}
#[doc = "`write(|w| ..)` method takes [`onemcu_cti_control::W`](W) writer structure"]
impl crate::Writable for OnemcuCtiControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ONEMCU_CTI_CONTROL to value 0"]
impl crate::Resettable for OnemcuCtiControlSpec {
    const RESET_VALUE: u32 = 0;
}
