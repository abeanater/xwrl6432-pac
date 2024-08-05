#[doc = "Register `SCITD` reader"]
pub type R = crate::R<ScitdSpec>;
#[doc = "Register `SCITD` writer"]
pub type W = crate::W<ScitdSpec>;
#[doc = "Field `TD` reader - 7:0\\]
Contains Data to be transmitted. This is pushed to SCITXSHF(shift register) when TXENA bit is set in SCRGCR1 register."]
pub type TdR = crate::FieldReader;
#[doc = "Field `TD` writer - 7:0\\]
Contains Data to be transmitted. This is pushed to SCITXSHF(shift register) when TXENA bit is set in SCRGCR1 register."]
pub type TdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Contains Data to be transmitted. This is pushed to SCITXSHF(shift register) when TXENA bit is set in SCRGCR1 register."]
    #[inline(always)]
    pub fn td(&self) -> TdR {
        TdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Contains Data to be transmitted. This is pushed to SCITXSHF(shift register) when TXENA bit is set in SCRGCR1 register."]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TdW<ScitdSpec> {
        TdW::new(self, 0)
    }
}
#[doc = "Transmit Data Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scitd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scitd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScitdSpec;
impl crate::RegisterSpec for ScitdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scitd::R`](R) reader structure"]
impl crate::Readable for ScitdSpec {}
#[doc = "`write(|w| ..)` method takes [`scitd::W`](W) writer structure"]
impl crate::Writable for ScitdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCITD to value 0"]
impl crate::Resettable for ScitdSpec {
    const RESET_VALUE: u32 = 0;
}
