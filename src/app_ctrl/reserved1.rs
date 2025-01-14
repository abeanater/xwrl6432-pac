#[doc = "Register `RESERVED1` reader"]
pub type R = crate::R<Reserved1Spec>;
#[doc = "Register `RESERVED1` writer"]
pub type W = crate::W<Reserved1Spec>;
#[doc = "Field `res` reader - 31:0\\]
reserved"]
pub type ResR = crate::FieldReader<u32>;
#[doc = "Field `res` writer - 31:0\\]
reserved"]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
reserved"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> ResW<Reserved1Spec> {
        ResW::new(self, 0)
    }
}
#[doc = "RESERVED1\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reserved1Spec;
impl crate::RegisterSpec for Reserved1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved1::R`](R) reader structure"]
impl crate::Readable for Reserved1Spec {}
#[doc = "`write(|w| ..)` method takes [`reserved1::W`](W) writer structure"]
impl crate::Writable for Reserved1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESERVED1 to value 0"]
impl crate::Resettable for Reserved1Spec {
    const RESET_VALUE: u32 = 0;
}
