#[doc = "Register `SCIPIO3` reader"]
pub type R = crate::R<Scipio3Spec>;
#[doc = "Register `SCIPIO3` writer"]
pub type W = crate::W<Scipio3Spec>;
#[doc = "Field `CLKOUT` reader - 0:0\\]
Reserved"]
pub type ClkoutR = crate::BitReader;
#[doc = "Field `CLKOUT` writer - 0:0\\]
Reserved"]
pub type ClkoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOUT` reader - 1:1\\]
Receive pin out. This bit is effective in LIN or SCI mode. This pin specifies the logic to be output on pin LINRX."]
pub type RxoutR = crate::BitReader;
#[doc = "Field `RXOUT` writer - 1:1\\]
Receive pin out. This bit is effective in LIN or SCI mode. This pin specifies the logic to be output on pin LINRX."]
pub type RxoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOUT` reader - 2:2\\]
Transmit pin out. This bit is effective in LIN or SCI mode. This pin specifies the logic to be output on pin LINTX."]
pub type TxoutR = crate::BitReader;
#[doc = "Field `TXOUT` writer - 2:2\\]
Transmit pin out. This bit is effective in LIN or SCI mode. This pin specifies the logic to be output on pin LINTX."]
pub type TxoutW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn clkout(&self) -> ClkoutR {
        ClkoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin out. This bit is effective in LIN or SCI mode. This pin specifies the logic to be output on pin LINRX."]
    #[inline(always)]
    pub fn rxout(&self) -> RxoutR {
        RxoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin out. This bit is effective in LIN or SCI mode. This pin specifies the logic to be output on pin LINTX."]
    #[inline(always)]
    pub fn txout(&self) -> TxoutR {
        TxoutR::new(((self.bits >> 2) & 1) != 0)
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
    pub fn clkout(&mut self) -> ClkoutW<Scipio3Spec> {
        ClkoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin out. This bit is effective in LIN or SCI mode. This pin specifies the logic to be output on pin LINRX."]
    #[inline(always)]
    #[must_use]
    pub fn rxout(&mut self) -> RxoutW<Scipio3Spec> {
        RxoutW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin out. This bit is effective in LIN or SCI mode. This pin specifies the logic to be output on pin LINTX."]
    #[inline(always)]
    #[must_use]
    pub fn txout(&mut self) -> TxoutW<Scipio3Spec> {
        TxoutW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio3Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "SCIPIO3\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio3Spec;
impl crate::RegisterSpec for Scipio3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio3::R`](R) reader structure"]
impl crate::Readable for Scipio3Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio3::W`](W) writer structure"]
impl crate::Writable for Scipio3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO3 to value 0"]
impl crate::Resettable for Scipio3Spec {
    const RESET_VALUE: u32 = 0;
}
