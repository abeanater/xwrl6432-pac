#[doc = "Register `SCIPIO0` reader"]
pub type R = crate::R<Scipio0Spec>;
#[doc = "Register `SCIPIO0` writer"]
pub type W = crate::W<Scipio0Spec>;
#[doc = "Field `CLKFUNC` reader - 0:0\\]
Reserved"]
pub type ClkfuncR = crate::BitReader;
#[doc = "Field `CLKFUNC` writer - 0:0\\]
Reserved"]
pub type ClkfuncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFUNC` reader - 1:1\\]
Receive pin function. This bit is effective in LIN or SCI mode. This bit defines the function of the LINRX pin."]
pub type RxfuncR = crate::BitReader;
#[doc = "Field `RXFUNC` writer - 1:1\\]
Receive pin function. This bit is effective in LIN or SCI mode. This bit defines the function of the LINRX pin."]
pub type RxfuncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFUNC` reader - 2:2\\]
Transmit pin function. This bit is effective in LIN or SCI mode. This bit defines the function of LINTX pin."]
pub type TxfuncR = crate::BitReader;
#[doc = "Field `TXFUNC` writer - 2:2\\]
Transmit pin function. This bit is effective in LIN or SCI mode. This bit defines the function of LINTX pin."]
pub type TxfuncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 31:16\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:16\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    pub fn clkfunc(&self) -> ClkfuncR {
        ClkfuncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin function. This bit is effective in LIN or SCI mode. This bit defines the function of the LINRX pin."]
    #[inline(always)]
    pub fn rxfunc(&self) -> RxfuncR {
        RxfuncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin function. This bit is effective in LIN or SCI mode. This bit defines the function of LINTX pin."]
    #[inline(always)]
    pub fn txfunc(&self) -> TxfuncR {
        TxfuncR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clkfunc(&mut self) -> ClkfuncW<Scipio0Spec> {
        ClkfuncW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin function. This bit is effective in LIN or SCI mode. This bit defines the function of the LINRX pin."]
    #[inline(always)]
    #[must_use]
    pub fn rxfunc(&mut self) -> RxfuncW<Scipio0Spec> {
        RxfuncW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin function. This bit is effective in LIN or SCI mode. This bit defines the function of LINTX pin."]
    #[inline(always)]
    #[must_use]
    pub fn txfunc(&mut self) -> TxfuncW<Scipio0Spec> {
        TxfuncW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio0Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "The SCIPIO0 register is used to enable the LINTX and LINRX pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio0Spec;
impl crate::RegisterSpec for Scipio0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio0::R`](R) reader structure"]
impl crate::Readable for Scipio0Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio0::W`](W) writer structure"]
impl crate::Writable for Scipio0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO0 to value 0"]
impl crate::Resettable for Scipio0Spec {
    const RESET_VALUE: u32 = 0;
}
