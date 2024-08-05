#[doc = "Register `TSCC` reader"]
pub type R = crate::R<TsccSpec>;
#[doc = "Register `TSCC` writer"]
pub type W = crate::W<TsccSpec>;
#[doc = "Field `TSS` reader - 1:0\\]
Timestamp Select"]
pub type TssR = crate::FieldReader;
#[doc = "Field `TSS` writer - 1:0\\]
Timestamp Select"]
pub type TssW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NU20` reader - 15:2\\]
Reserved"]
pub type Nu20R = crate::FieldReader<u16>;
#[doc = "Field `NU20` writer - 15:2\\]
Reserved"]
pub type Nu20W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `TCP` reader - 19:16\\]
Timestamp Counter Prescaler"]
pub type TcpR = crate::FieldReader;
#[doc = "Field `TCP` writer - 19:16\\]
Timestamp Counter Prescaler"]
pub type TcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU21` reader - 31:20\\]
Reserved"]
pub type Nu21R = crate::FieldReader<u16>;
#[doc = "Field `NU21` writer - 31:20\\]
Reserved"]
pub type Nu21W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu20(&self) -> Nu20R {
        Nu20R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TcpR {
        TcpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn nu21(&self) -> Nu21R {
        Nu21R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Timestamp Select"]
    #[inline(always)]
    #[must_use]
    pub fn tss(&mut self) -> TssW<TsccSpec> {
        TssW::new(self, 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu20(&mut self) -> Nu20W<TsccSpec> {
        Nu20W::new(self, 2)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Timestamp Counter Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn tcp(&mut self) -> TcpW<TsccSpec> {
        TcpW::new(self, 16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu21(&mut self) -> Nu21W<TsccSpec> {
        Nu21W::new(self, 20)
    }
}
#[doc = "TSCC\n\nYou can [`read`](crate::Reg::read) this register and get [`tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TsccSpec;
impl crate::RegisterSpec for TsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscc::R`](R) reader structure"]
impl crate::Readable for TsccSpec {}
#[doc = "`write(|w| ..)` method takes [`tscc::W`](W) writer structure"]
impl crate::Writable for TsccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCC to value 0"]
impl crate::Resettable for TsccSpec {
    const RESET_VALUE: u32 = 0;
}
