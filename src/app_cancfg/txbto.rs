#[doc = "Register `TXBTO` reader"]
pub type R = crate::R<TxbtoSpec>;
#[doc = "Register `TXBTO` writer"]
pub type W = crate::W<TxbtoSpec>;
#[doc = "Field `TO` reader - 31:0\\]
Transmission Occurred"]
pub type ToR = crate::FieldReader<u32>;
#[doc = "Field `TO` writer - 31:0\\]
Transmission Occurred"]
pub type ToW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Transmission Occurred"]
    #[inline(always)]
    pub fn to(&self) -> ToR {
        ToR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Transmission Occurred"]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> ToW<TxbtoSpec> {
        ToW::new(self, 0)
    }
}
#[doc = "TXBTO\n\nYou can [`read`](crate::Reg::read) this register and get [`txbto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbtoSpec;
impl crate::RegisterSpec for TxbtoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbto::R`](R) reader structure"]
impl crate::Readable for TxbtoSpec {}
#[doc = "`write(|w| ..)` method takes [`txbto::W`](W) writer structure"]
impl crate::Writable for TxbtoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXBTO to value 0"]
impl crate::Resettable for TxbtoSpec {
    const RESET_VALUE: u32 = 0;
}
