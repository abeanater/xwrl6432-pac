#[doc = "Register `SCIPIO7` reader"]
pub type R = crate::R<Scipio7Spec>;
#[doc = "Register `SCIPIO7` writer"]
pub type W = crate::W<Scipio7Spec>;
#[doc = "Field `CLKPD` reader - 0:0\\]
Reserved"]
pub type ClkpdR = crate::BitReader;
#[doc = "Field `CLKPD` writer - 0:0\\]
Reserved"]
pub type ClkpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPD` reader - 1:1\\]
Receive pin pull control disable. This bit is effective in LIN or SCI mode. This bit disables pull control capability on the input pin LINRX."]
pub type RxpdR = crate::BitReader;
#[doc = "Field `RXPD` writer - 1:1\\]
Receive pin pull control disable. This bit is effective in LIN or SCI mode. This bit disables pull control capability on the input pin LINRX."]
pub type RxpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPD` reader - 2:2\\]
Transmit pin pull control disable. This bit is effective in LIN or SCI mode. This bit disables pull control capability on the input pin LINTX."]
pub type TxpdR = crate::BitReader;
#[doc = "Field `TXPD` writer - 2:2\\]
Transmit pin pull control disable. This bit is effective in LIN or SCI mode. This bit disables pull control capability on the input pin LINTX."]
pub type TxpdW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn clkpd(&self) -> ClkpdR {
        ClkpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin pull control disable. This bit is effective in LIN or SCI mode. This bit disables pull control capability on the input pin LINRX."]
    #[inline(always)]
    pub fn rxpd(&self) -> RxpdR {
        RxpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin pull control disable. This bit is effective in LIN or SCI mode. This bit disables pull control capability on the input pin LINTX."]
    #[inline(always)]
    pub fn txpd(&self) -> TxpdR {
        TxpdR::new(((self.bits >> 2) & 1) != 0)
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
    pub fn clkpd(&mut self) -> ClkpdW<Scipio7Spec> {
        ClkpdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Receive pin pull control disable. This bit is effective in LIN or SCI mode. This bit disables pull control capability on the input pin LINRX."]
    #[inline(always)]
    #[must_use]
    pub fn rxpd(&mut self) -> RxpdW<Scipio7Spec> {
        RxpdW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transmit pin pull control disable. This bit is effective in LIN or SCI mode. This bit disables pull control capability on the input pin LINTX."]
    #[inline(always)]
    #[must_use]
    pub fn txpd(&mut self) -> TxpdW<Scipio7Spec> {
        TxpdW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio7Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "SCIPIO7\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio7Spec;
impl crate::RegisterSpec for Scipio7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio7::R`](R) reader structure"]
impl crate::Readable for Scipio7Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio7::W`](W) writer structure"]
impl crate::Writable for Scipio7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO7 to value 0"]
impl crate::Resettable for Scipio7Spec {
    const RESET_VALUE: u32 = 0;
}
